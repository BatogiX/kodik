use crate::Error;

/// Extracts the domain from a URL.
///
/// # Errors
///
/// Returns `KodikError::Regex` if no valid domain is found in the URL.
pub fn extract_domain(url: &str) -> crate::Result<&str> {
    let domain_re = lazy_regex::regex!(r"(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z0-9][a-z0-9-]{0,61}[a-z0-9]");

    log::debug!("Extracting domain...");

    let domain = domain_re
        .find(url)
        .ok_or_else(|| Error::RegexMatch(format!("no valid domain found in '{url}'")))?
        .as_str();

    log::trace!("Extracted domain: {domain}");

    Ok(domain)
}

/// Extracts the anime ID from a Shikimori URL.
///
/// # Errors
///
/// Returns an error if the URL does not contain a valid anime ID.
pub fn extract_anime_id(url: &str) -> crate::Result<&str> {
    let id_re = lazy_regex::regex!(r"/animes?/(?:[a-z])?([0-9]+)(?:-|$|/)");

    id_re
        .captures(url)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .ok_or_else(|| Error::RegexMatch(format!("id not found in '{url}'")))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    #[test]
    fn getting_domain() {
        let url_with_scheme = "https://kodikplayer.com/video/91873/060cab655974d46835b3f4405807acc2/720p";
        let url_without_scheme = "kodikplayer.com/video/91873/060cab655974d46835b3f4405807acc2/720p";

        assert_eq!("kodikplayer.com", extract_domain(url_with_scheme).unwrap());
        assert_eq!("kodikplayer.com", extract_domain(url_without_scheme).unwrap());
    }

    #[test]
    fn extract_anime_id_shiki_test() {
        let url = "https://shikimori.io/animes/431-howl-no-ugoku-shiro";
        let expected = "431";

        let id = extract_anime_id(url).unwrap();
        assert_eq!(id, expected);
    }

    #[test]
    fn extract_anime_id_shiki_alt_test() {
        let urls: [&str; 3] = [
            "https://shikimori.io/animes/z199-sen-to-chihiro-no-kamikakushi",
            "https://shikimori.io/animes/y28851-koe-no-katachi",
            "https://shikimori.io/animes/x16782-kotonoha-no-niwa",
        ];
        let expected: [&str; 3] = ["199", "28851", "16782"];

        for (url, expected) in urls.into_iter().zip(expected) {
            let id = extract_anime_id(url).unwrap();
            assert_eq!(id, expected);
        }
    }

    #[test]
    fn extract_anime_id_mal_test() {
        let url = "https://myanimelist.net/anime/199/Sen_to_Chihiro_no_Kamikakushi";
        let expected = "199";

        let id = extract_anime_id(url).unwrap();
        assert_eq!(id, expected);
    }
}
