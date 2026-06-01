//! # Kodik Parser library.
//! `kodik-parser` for getting direct links to files from Kodik.

mod api;
mod decoder;
mod error;
mod parser;
mod scraper;
mod state;

pub use api::{KodikApiResponse, TranslationType};
pub use error::{Error, Result};
pub use parser::Links;
pub use state::KODIK_STATE;
