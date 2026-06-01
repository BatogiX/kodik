//! # Kodik Parser library.
//! `kodik-parser` for getting direct links to files from Kodik.

mod api;
mod decoder;
mod error;
mod parser;
mod scraper;
mod state;

pub use api::{KodikApiResponse, TranslationType, fetch_shiki_kodik_videos};
pub use error::{Error, Result};
pub use parser::parse;
pub use scraper::{Link, Links};
pub use state::KODIK_STATE;
