use reqwest::cookie::Jar;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

type Result<T, E = crate::Error> = std::result::Result<T, E>;

pub fn load_netscape_cookies(path: &impl AsRef<Path>) -> Result<Jar> {
    let jar = Jar::default();
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    while reader.read_line(&mut line)? > 0 {
        let trimmed = line.trim();

        if trimmed.starts_with('#') || trimmed.is_empty() {
            line.clear();
            continue;
        }

        let mut parts = trimmed.splitn(7, '\t');
        let domain = parts.next().ok_or_else(|| crate::Error::NotFound("malformed cookie: missing domain".into()))?;
        let key = parts.nth(4).ok_or_else(|| crate::Error::NotFound("malformed cookie: missing name".into()))?;
        let value = parts.next().ok_or_else(|| crate::Error::NotFound("malformed cookie: missing value".into()))?;

        let mut cookie = String::with_capacity(key.len() + value.len() + domain.len() + 10);
        cookie.push_str(key);
        cookie.push('=');
        cookie.push_str(value);
        cookie.push_str("; Domain=");
        cookie.push_str(domain);

        let domain = domain.trim_start_matches('.');
        let mut url_str = String::with_capacity(8 + domain.len());
        url_str.push_str("https://");
        url_str.push_str(domain);

        jar.add_cookie_str(
            &cookie,
            &reqwest::Url::parse(&url_str)
                .map_err(|e| crate::Error::NotFound(format!("invalid cookie domain '{domain}': {e}")))?,
        );

        line.clear();
    }

    Ok(jar)
}
