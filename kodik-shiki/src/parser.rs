use kodik_utils::Error;

/// Extracts the anime ID from a Shikimori URL.
///
/// # Errors
///
/// Returns an error if the URL does not contain a valid anime ID.
pub fn extract_id(url: &str) -> Result<&str, Error> {
    let id_re = lazy_regex::regex!(r"/animes?/(?:[a-z])?([0-9]+)(?:-|$|/)");

    id_re
        .captures(url)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str())
        .ok_or(Error::RegexMatch(format!("id not found in '{url}'")))
}
