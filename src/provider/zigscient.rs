use super::{AssetInfo, LspProvider, Result};
use zed_extension_api::{Architecture, Os};

#[derive(Debug, Default)]
pub struct Zigscient;

impl LspProvider for Zigscient {
    fn config(&self) -> (&'static str, &'static str) {
        ("zigscient", "nuIIpointerexception/zigscient-builds")
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

    fn binary_name(&self, name: &str, platform_key: &str) -> String {
        format!("{}-{}", name, platform_key)
    }
}