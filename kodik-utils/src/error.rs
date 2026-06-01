//! Error types for the Kodik library.
use reqwest::header;
use std::string;
use thiserror::Error as ThisError;

/// Errors from kodik.
#[derive(ThisError, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Reqwest HTTP client error.
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// IO error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Base64 decoding error.
    #[error(transparent)]
    Decode(#[from] base64::DecodeError),

    /// UTF-8 conversion error.
    #[error(transparent)]
    FromUtf8(#[from] string::FromUtf8Error),

    /// Invaliad header value
    #[error(transparent)]
    InvalidHeaderValue(#[from] header::InvalidHeaderValue),

    #[error(transparent)]
    Regex(#[from] lazy_regex::regex::Error),

    #[error(transparent)]
    SerdeYaml(#[from] serde_saphyr::Error),

    /// Regex matching error.
    #[error("{0}")]
    RegexMatch(String),

    /// Not found error.
    #[error("{0}")]
    NotFound(String),
}
