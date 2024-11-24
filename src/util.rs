use crate::error::{Error, Result};
use zed_extension_api::{
    http_client,
    http_client::{HttpMethod, HttpRequestBuilder},
    serde_json::{self, Value},
};

/// URL encodes text with special handling for version number characters
///
/// * ASCII alphanumeric characters are preserved
/// * All other characters are percent-encoded
///
/// We need this because zls expects version numbers to be encoded
pub fn url_encode(text: &str) -> String {
    let mut encoded = String::with_capacity(text.len() * 3);
    for c in text.chars() {
        match c {
            '+' => encoded.push_str("%2B"),
            '-' => encoded.push_str("%2D"),
            '.' => encoded.push_str("%2E"),
            c if c.is_ascii_alphanumeric() => encoded.push(c),
            c => {
                let bytes = c.to_string().into_bytes();
                for b in bytes {
                    encoded.push_str(&format!("%{:02X}", b));
                }
            }
        }
    }
    encoded
}

/// Fetches and parses JSON from a given URL
///
/// Makes a GET request to the provided URL and attempts to parse the response as JSON
pub fn fetch_json(url: &str) -> Result<serde_json::Value> {
    let request = HttpRequestBuilder::new()
        .method(HttpMethod::Get)
        .url(url)
        .build()
        .map_err(|e| Error::FetchFailed { url: url.to_string(), error: e.to_string() })?;
    let response = http_client::fetch(&request)
        .map_err(|e| Error::FetchFailed { url: url.to_string(), error: e.to_string() })?;
    serde_json::from_slice(&response.body)
        .map_err(|e| format!("Failed to parse JSON response: {}", e))
}

/// Quick hack to validate url from our config
///
pub fn parse_url(url: &str) -> Result<()> {
    let error = || Error::Configuration {
        message: "Invalid URL".into(),
        fix: "Please provide a valid URL starting with http:// or https://".into(),
    };

    let parts: Vec<&str> =
        url.trim().split(|c| c == '#' || c == '?').next().ok_or_else(error)?.split("://").collect();

    if parts.len() != 2 || !["http", "https"].contains(&parts[0]) {
        return Err("Invalid URL".into());
    }
    Ok(())
}

/// Extension trait for serde_json::Value to simplify JSON field extraction
///
/// Provides convenient methods to extract typed values from JSON with proper error handling.
pub trait JsonExt {
    /// Extracts a string field from a JSON value
    fn get_str(&self, field: &str) -> Result<&str>;
    
    /// Extracts a nested string field using dot notation (e.g. "parent.child")
    fn get_nested_str(&self, path: &str) -> Result<&str>;
    
    /// Extracts an object field from a JSON value
    fn get_obj(&self, field: &str) -> Result<&serde_json::Map<String, Value>>;
}

impl JsonExt for serde_json::Value {
    fn get_str(&self, field: &str) -> Result<&str> {
        self.get(field)
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("Missing field: {}", field))
    }

    fn get_nested_str(&self, path: &str) -> Result<&str> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = self;
        
        for (i, &part) in parts.iter().enumerate() {
            current = current.get(part).ok_or_else(|| format!("Missing field: {}", parts[..=i].join(".")))?;
        }
        
        current.as_str().ok_or_else(|| format!("Missing field: {}", path))
    }

    fn get_obj(&self, field: &str) -> Result<&serde_json::Map<String, Value>> {
        self.get(field)
            .and_then(|v| v.as_object())
            .ok_or_else(|| format!("Missing field: {}", field))
    }
}


