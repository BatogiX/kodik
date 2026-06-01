use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    KodikUtils(#[from] kodik_utils::Error),

    /// Link cannot be decoded error.
    #[error("link cannot be decoded {0}")]
    LinkCannotBeDecoded(String),

    #[error("invalid `translation_type`: `{0}`, expected voice or subtitles")]
    InvalidTranslationType(String),
}

impl From<lazy_regex::regex::Error> for Error {
    fn from(e: lazy_regex::regex::Error) -> Self {
        Self::KodikUtils(kodik_utils::Error::Regex(e))
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Self::KodikUtils(kodik_utils::Error::Decode(e))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::KodikUtils(kodik_utils::Error::FromUtf8(e))
    }
}
