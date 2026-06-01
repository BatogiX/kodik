use kodik_utils::{Client, ClientExt as _};
use serde::Deserialize;

use crate::{AnimeKind, AnimeStatus, UserRate};

#[derive(Debug, Deserialize)]
pub struct ShikiApiAnimes {
	pub id: usize,
	pub name: String,
	// russian: String,
	// url: String,
	pub kind: AnimeKind,
	// score: String,
	pub status: AnimeStatus,
	pub episodes: usize,
	pub episodes_aired: usize,
	// aired_on: String,
	// released_on: String,
	// rating: String,
	pub franchise: Option<String>,
	pub user_rate: Option<UserRate>,
}

impl ShikiApiAnimes {
	/// Fetches anime details from the Shikimori API.
	///
	/// # Errors
	///
	/// Returns an error if the API request fails or the response cannot be deserialized.
	pub async fn fetch(client: &Client, url: &str) -> crate::Result<Self> {
		let anime_id = kodik_utils::extract_anime_id(url)?;
		let domain = kodik_utils::extract_domain(url)?;
		let url = kodik_utils::api_url(domain, &format!("/api/animes/{anime_id}"));
		let shiki_api_animes = client.fetch_as_json(&url).await?;

		Ok(shiki_api_animes)
	}
}

#[derive(Debug, Deserialize)]
pub struct ShikiApiUsersWhoami {
	pub id: Option<usize>,
}

impl ShikiApiUsersWhoami {
	/// # Errors
	///
	/// Returns an error if the API request fails or the response cannot be deserialized.
	pub async fn fetch(client: &Client, host: &str) -> crate::Result<Self> {
		let whoami = client
			.fetch_as_json(&kodik_utils::api_url(host, "/api/users/whoami"))
			.await?;

		Ok(whoami)
	}
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn fetch_shiki_api_animes_test() {
		let client = Client::new();
		let url = "https://shikimori.net/animes/33";

		dbg!(ShikiApiAnimes::fetch(&client, url).await.unwrap());
	}
}
