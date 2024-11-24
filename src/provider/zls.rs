use super::{version, AssetInfo, LspProvider, Result};
use crate::{settings::VersionSource, util::{url_encode, JsonExt}};
use zed_extension_api::{Architecture, Os,  serde_json::Value};
use crate::util::fetch_json;

#[derive(Debug, Default)]
pub struct Zls;

impl LspProvider for Zls {
    fn config(&self) -> (&'static str, &'static str) {
        ("zls", "zigtools/zls")
    }

    fn asset_info(
        &self,
        name: &str,
        platform: Os,
        arch: Architecture,
        _version: &str,
    ) -> Result<AssetInfo> {
        let arch_str = match arch {
            Architecture::Aarch64 => "aarch64",
            Architecture::X8664 => "x86_64",
            Architecture::X86 => "x86",
        };

        let platform_str = match platform {
            Os::Mac => "macos",
            Os::Linux => "linux",
            Os::Windows => "windows",
        };

        let platform_key = format!("{}-{}", arch_str, platform_str);
        
        let zig_response: Value = fetch_json("https://ziglang.org/download/index.json")?;
        let zig_version = zig_response.get_nested_str("master.version")?;
        let endpoint_url = format!(
            "{}?zig_version={}&compatibility=only-runtime",
            "https://releases.zigtools.org/v1/zls/select-version",
            url_encode(zig_version)
        );
        let version_info = version::fetch_version(&VersionSource::ApiEndpoint { url: endpoint_url }, &platform_key)?;

        Ok(AssetInfo {
            name: format!("{}-{}", name, version_info.version),
            url: Some(version_info.download_url),
        })
    }

    fn binary_name(&self, name: &str, _platform_key: &str) -> String {
        name.to_string()
    }
}
