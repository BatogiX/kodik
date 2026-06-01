mod client_ext;
mod cookies;
mod error;
mod re;

pub use client_ext::ClientExt;
pub use cookies::load_netscape_cookies;
pub use error::Error;
pub use re::{extract_anime_id, extract_domain};
pub use reqwest::Client;

/// Builds a URL from a host and path (path should start with `/`).
#[must_use]
pub fn api_url(host: &str, path: &str) -> String {
    format!("https://{host}{path}")
}
