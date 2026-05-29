use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum UserRateStatus {
    Planned,
    Watching,
    Rewatching,
    Completed,
    OnHold,
    Dropped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimeStatus {
    Anons,
    Ongoing,
    Released,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct UserRate {
    pub episodes: usize,
    #[serde(deserialize_with = "deserialize_usize_from_string_or_number")]
    pub id: usize,
    pub rewatches: usize,
    pub status: UserRateStatus,
}

impl UserRate {
    #[must_use]
    pub const fn new(id: usize, status: UserRateStatus, episodes: usize, rewatches: usize) -> Self {
        Self {
            episodes,
            id,
            rewatches,
            status,
        }
    }
}

pub fn deserialize_usize_from_string_or_number<'de, D>(deserializer: D) -> std::result::Result<usize, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Number(usize),
    }

    match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::Number(n) => Ok(n),
        StringOrNumber::String(s) => s.parse::<usize>().map_err(serde::de::Error::custom),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AnimeKind {
    Tv,
    Movie,
    Ova,
    Ona,
    Special,
    TvSpecial,
    Music,
    Pv,
    Cm,
}

impl AnimeKind {
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Tv => "TV Series",
            Self::Movie => "Movie",
            Self::Ova => "OVA",
            Self::Ona => "ONA",
            Self::Special => "Special",
            Self::TvSpecial => "TV Special",
            Self::Music => "Music",
            Self::Pv => "PV",
            Self::Cm => "CM",
        }
    }
}
