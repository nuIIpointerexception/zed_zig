use crate::error::{Error, Result};
use crate::util::parse_url;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
    #[serde(alias = "zigscient")]
    Zigscient,
    #[serde(alias = "zls")]
    Zls,
    #[serde(alias = "zls-stable")]
    ZlsStable,
    #[serde(alias = "custom")]
    Custom,
}

impl Default for Provider {
    fn default() -> Self {
        Self::Zls
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    Github,
    Api,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum VersionSource {
    #[serde(rename = "github")]
    GitHub {
        url: String,
        #[serde(default)]
        pre_release: bool,
    },
    #[serde(rename = "api")]
    ApiEndpoint { url: String },
}

impl VersionSource {
    pub fn validate(&self) -> Result<()> {
        match self {
            Self::GitHub { url, .. } if !url.contains('/') || url.matches('/').count() != 1 => {
                Err(Error::Configuration {
                    message: "Invalid GitHub repository format".into(),
                    fix: "Please provide the repository in the format 'owner/repo' (e.g. 'zigtools/zls')".into(),
                }.into())
            }
            Self::ApiEndpoint { url } if parse_url(url).is_err() => {
                Err(Error::Configuration {
                    message: format!("Invalid URL: {}", url),
                    fix: "Please provide a valid URL starting with http:// or https://".into(),
                }.into())
            }
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ZigTooling {
    #[serde(default)]
    pub provider: Provider,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    pub version_source: Option<VersionSource>,
}

impl Default for ZigTooling {
    fn default() -> Self {
        Self { provider: Provider::default(), path: None, args: None, version_source: None }
    }
}
