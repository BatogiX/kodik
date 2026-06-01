mod client_ext;
mod error;
mod re;

pub use client_ext::ClientExt;
pub use error::Error;
pub use re::{extract_anime_id, extract_domain};
pub use reqwest::Client;
