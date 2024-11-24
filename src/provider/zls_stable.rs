use super::{AssetInfo, LspProvider, Result};
use zed_extension_api::{Architecture, Os};

#[derive(Debug, Default)]
pub struct ZlsStable;

impl LspProvider for ZlsStable {
    fn config(&self) -> (&'static str, &'static str) {
        ("zls", "zigtools/zls")
    }

    fn asset_info(
        &self,
        name: &str,
        _platform: Os,
        _arch: Architecture,
        version: &str,
    ) -> Result<AssetInfo> {

        Ok(AssetInfo {
            name: format!("{}-{}", name, version),
            url: None,
        })
    }

    fn binary_name(&self, name: &str, _platform_key: &str) -> String {
        name.to_string()
    }
}
