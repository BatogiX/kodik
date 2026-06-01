use kodik_utils::{Client, ClientExt as _};
use serde::Serialize;

use crate::{UserRate, UserRateStatus};

#[derive(Debug, Serialize)]
pub struct ShikiApiUserRates {
    user_rate: ShikiApiUserRatesUserRate,
}

impl ShikiApiUserRates {
    #[must_use]
    pub const fn new(
        episodes: usize,
        rewatches: usize,
        status: UserRateStatus,
        target_id: usize,
        target_type: UserRatesTargetType,
        user_id: usize,
    ) -> Self {
        Self {
            user_rate: ShikiApiUserRatesUserRate::new(episodes, rewatches, status, target_id, target_type, user_id),
        }
    }

    pub async fn patch(&self, client: &Client, host: &str, user_rate_id: usize) -> crate::Result<UserRate> {
        let user_rate = client
            .patch_json_as_json(&format!("https://{host}/api/v2/user_rates/{user_rate_id}"), self)
            .await?;

        Ok(user_rate)
    }

    pub async fn post(&self, client: &Client, host: &str) -> crate::Result<UserRate> {
        let user_rate = client
            .post_json_as_json(&format!("https://{host}/api/v2/user_rates"), self)
            .await?;

        Ok(user_rate)
    }
}

#[derive(Debug, Serialize)]
struct ShikiApiUserRatesUserRate {
    pub episodes: usize,
    pub rewatches: usize,
    pub status: UserRateStatus,
    pub target_id: usize,
    pub target_type: UserRatesTargetType,
    pub user_id: usize,
}

impl ShikiApiUserRatesUserRate {
    const fn new(
        episodes: usize,
        rewatches: usize,
        status: UserRateStatus,
        target_id: usize,
        target_type: UserRatesTargetType,
        user_id: usize,
    ) -> Self {
        Self {
            episodes,
            rewatches,
            status,
            target_id,
            target_type,
            user_id,
        }
    }
}

#[derive(Debug, Serialize)]
pub enum UserRatesTargetType {
    Anime,
    // Manga,
    // VisualNovel,
}
