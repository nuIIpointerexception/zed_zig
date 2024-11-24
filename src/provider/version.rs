use crate::error::{Error, Result};
use crate::settings::VersionSource;
use crate::util::{fetch_json, JsonExt};
use zed_extension_api::{self as zed, serde_json::Value};

#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: String,
    pub download_url: String,
}

pub fn fetch_version(source: &VersionSource, platform: &str) -> Result<VersionInfo> {
    match source {
        VersionSource::GitHub { url, pre_release } => fetch_github_version(url, *pre_release, platform),
        VersionSource::ApiEndpoint { url } => fetch_api_version(url, platform),
    }
}

fn fetch_github_version(url: &str, pre_release: bool, platform: &str) -> Result<VersionInfo> {
    let release = zed::latest_github_release(
        url,
        zed::GithubReleaseOptions { require_assets: true, pre_release },
    )
    .map_err(|e| Error::LanguageServer(e.to_string()))?;

    let asset = release
        .assets
        .iter()
        .find(|a| a.name.contains(platform))
        .ok_or_else(|| Error::AssetNotFound(platform.to_string()))?;

    Ok(VersionInfo {
        version: release.version,
        download_url: asset.download_url.clone(),
    })
}

fn fetch_api_version(url: &str, platform: &str) -> Result<VersionInfo> {
    let response: Value = fetch_json(url)?;
    
    Ok(VersionInfo {
        version: response.get_str("version")?.to_string(),
        download_url: response
            .get_obj(platform)?
            .get("tarball")
            .and_then(|t| t.as_str())
            .ok_or_else(|| Error::Missing { field: format!("{}.tarball", platform) })?
            .to_string(),
    })
}
