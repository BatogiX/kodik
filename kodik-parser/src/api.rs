use std::{
    collections::BTreeMap,
    fmt::{Debug, Display},
    str::FromStr,
};

use kodik_utils::{Client, ClientExt as _};
use lazy_regex::Regex;
use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct KodikApiResponse {
    pub results: Results,
}

impl KodikApiResponse {
    /// Returns a `KodikApiResponse` for the given Shikimori anime ID.
    ///
    /// # Errors
    ///
    /// Returns a `KodikError` if the HTTP request fails or the response cannot be deserialized.
    pub async fn fetch_shiki(client: &Client, shikimori_id: usize) -> crate::Result<Self> {
        let query = format!("shikimori_id={shikimori_id}");
        Self::fetch(client, &query).await
    }

    /// Finds a result matching the given translation title or type.
    ///
    /// # Errors
    ///
    /// Returns an error if no video sources are found or the title regex is invalid.
    pub fn find_result(
        &self,
        translation_title: Option<&str>,
        translation_type: Option<TranslationType>,
    ) -> crate::Result<&SearchResult> {
        if let Some(translation_title) = translation_title {
            let title_re = Regex::new(&format!(r"(?i).*{translation_title}.*"))?;

            if let Some(result) = self.results.iter().find(|r| title_re.is_match(&r.translation.title)) {
                log::info!("found translation title '{}'", result.translation.title);
                return Ok(result);
            }

            log::warn!("no video source with title '{translation_title}'");
        }

        if let Some(translation_type) = translation_type {
            if let Some(result) = self.results.iter().find(|r| r.translation.r#type == translation_type) {
                log::info!("found translation title '{}'", result.translation.title);
                return Ok(result);
            }

            log::warn!("no video source with type '{translation_type}'");
        }

        let result = self
            .results
            .first()
            .ok_or_else(|| kodik_utils::Error::NotFound("no video sources found".to_owned()))?;

        log::info!("found first translation with title '{}'", result.translation.title);

        Ok(result)
    }

    async fn fetch(client: &Client, query: &str) -> crate::Result<Self> {
        const TOKEN: &str = env!("KODIK_TOKEN");

        let url = format!("https://kodik-api.com/search?token={TOKEN}&{query}&with_seasons=true&with_episodes=true");
        let kodik_api_response = client.fetch_as_json(&url).await?;

        Ok(kodik_api_response)
    }
}

pub type Results = Vec<SearchResult>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct SearchResult {
    pub link: String,
    pub title: String,
    pub translation: Translation,
    pub seasons: Option<BTreeMap<usize, Season>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Translation {
    pub title: String,
    pub r#type: TranslationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TranslationType {
    Voice,
    Subtitles,
}

impl Display for TranslationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Voice => write!(f, "voice"),
            Self::Subtitles => write!(f, "subtitles"),
        }
    }
}

impl FromStr for TranslationType {
    type Err = crate::Error;

    fn from_str(value: &str) -> crate::Result<Self> {
        match value.trim() {
            "voice" => Ok(Self::Voice),
            "subtitles" => Ok(Self::Subtitles),
            value => Err(crate::Error::InvalidTranslationType(value.to_owned())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Season {
    pub episodes: BTreeMap<usize, String>,
}
