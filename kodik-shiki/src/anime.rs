use crate::{ShikiApiAnimes, extract_id, parser};
use kodik_utils::{Client, Error, GET, extract_domain};

/// Fetches the user's episode progress for an anime from Shikimori.
///
/// # Errors
///
/// Returns an error if the URL is invalid, the ID cannot be extracted, or the API request fails.
pub async fn fetch_user_rate(client: &Client, url: &str) -> Result<Option<usize>, Error> {
    let domain = kodik_utils::extract_domain(url)?;
    let id = parser::extract_id(url)?;
    let url = format!("https://{domain}/api/animes/{id}");
    let shiki_api_animes: ShikiApiAnimes = client.fetch_as_json(&url).await?;

    Ok(shiki_api_animes.user_rate.map(|ur| ur.episodes))
}

/// Fetches anime details from the Shikimori API.
///
/// # Errors
///
/// Returns an error if the API request fails or the response cannot be deserialized.
pub async fn fetch_shiki_api_animes(client: &Client, url: &str) -> Result<ShikiApiAnimes, Error> {
    let anime_id = extract_id(url)?;
    let domain = extract_domain(url)?;
    let url = format!("https://{domain}/api/animes/{anime_id}");
    let shiki_api_animes = client.fetch_as_json(&url).await?;

    Ok(shiki_api_animes)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_shiki_api_animes_test() {
        let client = Client::new();
        let url = "https://shikimori.net/animes/33";

        dbg!(fetch_shiki_api_animes(&client, url).await.unwrap());
    }
}
