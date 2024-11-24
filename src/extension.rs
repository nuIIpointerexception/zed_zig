use crate::{
    error::{Error, Result},
    provider::{
        fetch_version, AssetInfo, Custom, LspProvider, VersionInfo, Zigscient, Zls, ZlsStable,
    },
    settings::{Provider, ZigTooling},
};
use std::{fs, path::PathBuf};
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, LanguageServerId};

#[derive(Debug)]
pub struct ZigExtension {
    cached_binary_path: Option<PathBuf>,
    current_settings: ZigTooling,
}

impl Default for ZigExtension {
    fn default() -> Self {
        Self { cached_binary_path: None, current_settings: ZigTooling::default() }
    }
}

impl ZigExtension {
    fn get_provider(&self) -> Box<dyn LspProvider> {
        match self.current_settings.provider {
            Provider::Zls => Box::new(Zls),
            Provider::Zigscient => Box::new(Zigscient),
            Provider::Custom => Box::new(Custom),
            Provider::ZlsStable => Box::new(ZlsStable),
        }
    }

    fn find_existing_binary(
        &self,
        binary_name: &str,
        worktree: &zed::Worktree,
    ) -> Option<ZigTooling> {
        let settings = &self.current_settings;

        // Check configured path first
        if let Some(path) = &settings.path {
            let path = PathBuf::from(path);
            if path.is_file() {
                return Some(ZigTooling {
                    provider: settings.provider,
                    path: Some(path.to_string_lossy().into()),
                    args: settings.args.clone(),
                    version_source: settings.version_source.clone(),
                });
            }
        }

        // Check PATH
        if let Some(path) = worktree.which(binary_name) {
            return Some(ZigTooling {
                provider: settings.provider,
                path: Some(path),
                args: settings.args.clone(),
                version_source: settings.version_source.clone(),
            });
        }

        // Check cached path
        if let Some(path) = &self.cached_binary_path {
            if path.is_file() {
                return Some(ZigTooling {
                    provider: settings.provider,
                    path: Some(path.to_string_lossy().into()),
                    args: settings.args.clone(),
                    version_source: settings.version_source.clone(),
                });
            }
        }

        None
    }

    fn download_binary(
        &self,
        provider: &Box<dyn LspProvider>,
        platform: zed::Os,
        language_server_id: &LanguageServerId,
    ) -> Result<PathBuf> {
        let (name, repo) = provider.config();

        let platform_key = format!(
            "{}-{}",
            match zed::current_platform().1 {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X8664 => "x86_64",
                _ => "x86",
            },
            match platform {
                zed::Os::Mac => "macos",
                zed::Os::Linux => "linux-gnu",
                zed::Os::Windows => "windows",
            }
        );

        let binary_name = provider.binary_name(name, &platform_key);

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let version_info = match self.current_settings.provider {
            Provider::Custom => {
                let version_source = self.current_settings.version_source.as_ref()
                    .ok_or_else(|| Error::Configuration {
                        message: "Custom provider requires version_source".to_string(),
                        fix: "Please specify either a 'github' or 'api' version source in your configuration".to_string(),
                    })?;
                fetch_version(version_source, &platform_key)?
            }
            _ => {
                let release = zed::latest_github_release(
                    repo,
                    zed::GithubReleaseOptions { require_assets: true, pre_release: false },
                )
                .map_err(|e| {
                    Error::LanguageServer(format!("Failed to fetch release from {repo}: {e}"))
                })?;

                VersionInfo {
                    version: release.version,
                    download_url: release
                        .assets
                        .iter()
                        .find(|a| a.name.contains(&platform_key))
                        .ok_or_else(|| {
                            Error::AssetNotFound(format!("No asset found for {platform_key}"))
                        })?
                        .download_url
                        .clone(),
                }
            }
        };

        let AssetInfo { name: asset_name, url, .. } = provider.asset_info(
            name,
            platform,
            zed::current_platform().1,
            &version_info.version,
        )?;

        let download_url = url.unwrap_or(version_info.download_url);
        if download_url.is_empty() {
            return Err("No download URL available".into());
        }

        let version_dir = PathBuf::from(&asset_name);
        let binary_path = version_dir.join(format!(
            "{binary_name}{}",
            if platform == zed::Os::Windows { ".exe" } else { "" }
        ));

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            fs::create_dir_all(&version_dir).map_err(|e| {
                Error::InstallationFailed(format!(
                    "Failed to create directory {}: {e}",
                    version_dir.display()
                ))
            })?;

            zed::download_file(
                &download_url,
                version_dir
                    .to_str()
                    .ok_or_else(|| Error::InstallationFailed("Invalid path".into()))?,
                if platform == zed::Os::Windows {
                    zed::DownloadedFileType::Zip
                } else {
                    zed::DownloadedFileType::GzipTar
                },
            )
            .map_err(|e| {
                Error::DownloadFailed(format!("Failed to download from {download_url}: {e}"))
            })?;

            zed::make_file_executable(&binary_path.to_string_lossy())?;

            // TODO(viable): For now leave everything in place, so it can be reused across different workspaces.
            // cleanup
            // let entries =
            //     fs::read_dir(".").map_err(|e| format!("Failed to list working directory {e}"))?;
            // for entry in entries {
            //     let entry = entry.map_err(|e| format!("Failed to load directory entry {e}"))?;
            //     if entry.file_name().to_str() != Some(version_dir.to_str().unwrap()) {
            //         fs::remove_dir_all(entry.path()).ok();
            //     }
            // }
        }

        Ok(binary_path)
    }

    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<ZigTooling> {
        if let Ok(lsp_settings) = LspSettings::for_worktree(language_server_id.as_ref(), worktree) {
            let mut config = ZigTooling {
                path: lsp_settings.binary.as_ref().and_then(|b| b.path.clone()),
                args: lsp_settings.binary.as_ref().and_then(|b| b.arguments.clone()),
                ..Default::default()
            };

            if let Some(settings) = lsp_settings.settings {
                if let Some(Ok(provider)) =
                    settings.get("provider").map(|v| serde_json::from_value(v.clone()))
                {
                    config.provider = provider;
                }

                if let Some(Ok(version_source)) =
                    settings.get("version_source").map(|v| serde_json::from_value(v.clone()))
                {
                    config.version_source = Some(version_source);
                }
            }

            self.current_settings = match (config.provider, &config.version_source) {
                (Provider::Custom, None) => {
                    return Err(Error::Configuration {
                        message: "Custom provider requires version_source configuration".into(),
                        fix: "Please add a version_source configuration with either 'github' or 'api' type".into(),
                    }.into())
                },
                (Provider::Custom, Some(vs)) => vs.validate().map(|_| config)?,
                _ => config,
            };
        }

        let provider = self.get_provider();
        let (name, _) = provider.config();

        let tooling = if let Some(binary) = self.find_existing_binary(name, worktree) {
            binary
        } else {
            let platform = zed::current_platform().0;
            let binary_path = self.download_binary(&provider, platform, language_server_id)?;

            self.cached_binary_path = Some(binary_path.clone());

            ZigTooling {
                provider: self.current_settings.provider,
                path: Some(binary_path.to_string_lossy().into()),
                args: self.current_settings.args.clone(),
                version_source: self.current_settings.version_source.clone(),
            }
        };

        Ok(tooling)
    }
}

impl zed::Extension for ZigExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;

        let environment = match zed::current_platform().0 {
            zed::Os::Mac | zed::Os::Linux => Some(worktree.shell_env()),
            zed::Os::Windows => None,
        };

        Ok(zed::Command {
            command: binary.path.ok_or("No binary path available")?,
            args: binary.args.unwrap_or_default(),
            env: environment.unwrap_or_default(),
        })
    }
}
