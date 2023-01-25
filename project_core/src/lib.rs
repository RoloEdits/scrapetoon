use anyhow::{bail, Context, Result};
use chrono::Utc;
use core::time;
use rand::Rng;
use reqwest::Response;
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, thread};

pub mod regex;
pub struct ResponseFactory {}

impl ResponseFactory {
    ///# Panics
    ///
    /// Will panic if there it can't connect to URL.
    ///
    /// # Errors
    ///
    pub async fn get(url: &str) -> Result<Response> {
        let mut rng = rand::thread_rng();

        let mut retries = 5;
        let mut wait = 1;

        let stop_rng = rng.gen_range(1..3);

        thread::sleep(Duration::from_secs(stop_rng));

        let response: Response = loop {
            let wait_rng = rng.gen_range(1..3);
            match reqwest::get(url).await {
                Err(_) => {
                    if retries > 0 {
                        retries -= 1;
                        thread::sleep(time::Duration::from_secs(wait + wait_rng));
                        wait *= wait_rng;
                    } else {
                        bail!("Cannot connect. Check URL: {url}");
                    }
                }
                Ok(ok) => break ok,
            }
        };

        Ok(response)
    }
}

pub struct BlockingResponseFactory {}

impl BlockingResponseFactory {
    ///# Panics
    ///
    /// Will panic if there it can't connect to URL.
    ///
    /// # Errors
    ///
    pub fn get(url: &str) -> Result<reqwest::blocking::Response> {
        let mut rng = rand::thread_rng();

        let mut retries = 5;
        let mut wait = 1;

        let stop_rng = rng.gen_range(1..3);

        thread::sleep(Duration::from_secs(stop_rng));

        let response = loop {
            let wait_rng = rng.gen_range(1..3);
            match reqwest::blocking::get(url) {
                Err(_) => {
                    if retries > 0 {
                        retries -= 1;
                        thread::sleep(time::Duration::from_secs(wait + wait_rng));
                        wait *= wait_rng;
                    } else {
                        bail!("Cannot connect. Check URL: {url}");
                    }
                }
                Ok(ok) => break ok,
            }
        };

        Ok(response)
    }
}

pub struct SeriesConfiguration<'a> {
    pub filename: &'a str,
    pub page_url: &'a str,
    pub episode_url: &'a str,
    pub episode_url_offset: u16,
}

#[must_use]
pub fn get_current_utc_date() -> String {
    Utc::now().date_naive().to_string()
}

/// # Panics
///
/// # Errors
pub fn create_date_folder(filepath: &str) -> Result<PathBuf> {
    let path = Path::new(filepath);

    let date_now = get_current_utc_date();

    let date_path = path.join(date_now);

    if !date_path
        .try_exists()
        .context("Failed to check is date folder already exists")?
    {
        fs::create_dir(&date_path).context("Create date folder")?;
    }

    Ok(date_path)
}

/// # Errors
pub fn path_enforcer(filepath: &str) -> Result<&Path> {
    let path = Path::new(filepath);

    if !path.try_exists().context("Check if given path exists")? {
        fs::create_dir(path).context("Create given path so it's existence is enforced")?;
    }

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // TODO: Need to mock

    #[test]
    #[ignore]
    fn should_create_date_folder() {
        let given_path = r"D:\temp";

        let date = get_current_utc_date();

        let result = create_date_folder(given_path)
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap();

        let test = format!("{given_path}\\{date}");

        assert_eq!(result, test);
    }

    #[test]
    #[ignore]
    fn should_create_valid_folder() {
        let given_path = r"D:\temp\temp";

        let result = path_enforcer(given_path).unwrap().to_str().unwrap();

        assert_eq!(result, given_path);
    }
}
