mod error;
mod extension;
mod provider;
mod settings;
mod util;

pub use extension::ZigExtension;
use zed_extension_api as zed;

zed::register_extension!(ZigExtension);
