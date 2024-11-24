use crate::error::Result;
use zed_extension_api as zed;

mod custom;
mod version;
mod zigscient;
mod zls;
mod zls_stable;

pub use custom::Custom;
pub use version::fetch_version;
pub use version::VersionInfo;
pub use zigscient::Zigscient;
pub use zls::Zls;
pub use zls_stable::ZlsStable;

#[derive(Debug, Clone)]
pub struct AssetInfo {
    pub name: String,
    pub url: Option<String>,
}

pub trait LspProvider {
    fn config(&self) -> (&'static str, &'static str);
    fn asset_info(
        &self,
        name: &str,
        platform: zed::Os,
        arch: zed::Architecture,
        version: &str,
    ) -> Result<AssetInfo>;
    
    fn binary_name(&self, name: &str, _platform_key: &str) -> String {
        format!("{}", name)
    }
}
