mod client_ext;
mod error;
mod jar_ext;
mod re;

pub use client_ext::ClientExt;
pub use error::{Error, Result};
pub use jar_ext::JarExt;
pub use re::{extract_anime_id, extract_domain};
pub use reqwest::{Client, cookie::Jar};

/// Builds a URL from a host and path (path should start with `/`).
#[must_use]
pub fn api_url(host: &str, path: &str) -> String {
    format!("https://{host}{path}")
}
