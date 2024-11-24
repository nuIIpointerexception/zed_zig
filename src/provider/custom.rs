use super::{AssetInfo, LspProvider, Result};
use zed_extension_api::{Architecture, Os};

pub struct Custom;

impl LspProvider for Custom {
    // FIXME(viable): Be able to add custom names. so its usable.
    fn config(&self) -> (&'static str, &'static str) {
        ("custom", "")
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
