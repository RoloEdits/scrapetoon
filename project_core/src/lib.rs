use chrono::Utc;
use core::time;
use rand::Rng;
use reqwest::{Error, Response};
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
    pub async fn get(url: &str) -> Result<Response, Error> {
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
                        panic!("Cannot connect. Check URL: {url}")
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

#[must_use]
/// # Panics
pub fn create_date_folder(filepath: &str) -> PathBuf {
    let path = Path::new(filepath);

    let date_now = get_current_utc_date();

    let date_path = path.join(date_now);

    if !date_path.try_exists().unwrap() {
        fs::create_dir(&date_path).expect("Create date folder");
    }

    date_path
}

#[must_use]
pub fn path_enforcer(filepath: &str) -> &Path {
    let path = Path::new(filepath);

    if !path.try_exists().expect("Check if given path exists") {
        fs::create_dir(path).expect("Create given path so it's existence is enforced");
    }

    path
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Need to mock

    #[test]
    fn should_create_date_folder() {
        let given_path = r"D:\temp";

        let date = get_current_utc_date();

        let result = create_date_folder(given_path)
            .into_os_string()
            .into_string()
            .unwrap();

        let test = format!("{given_path}\\{date}");

        assert_eq!(result, test);
    }

    #[test]
    fn should_create_valid_folder() {
        let given_path = r"D:\temp\temp";

        let result = path_enforcer(given_path).to_str().unwrap();

        assert_eq!(result, given_path);
    }
}
