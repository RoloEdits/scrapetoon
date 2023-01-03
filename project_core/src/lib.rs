use chrono::Utc;
use core::time;
use reqwest::{Error, Response};
use std::{env, fs, thread};

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
        let mut retries = 5;
        let mut wait = 1;

        let response: Response = loop {
            match reqwest::get(url).await {
                Err(_) => {
                    if retries > 0 {
                        retries -= 1;
                        thread::sleep(time::Duration::from_secs(wait));
                        wait *= 2;
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

#[must_use]
pub fn create_date_folder(filepath: &str) -> String {
    let mut final_path = filepath;

    // Equalizes all paths so that an operation to add slashes can be done without worry of doubling up.
    if final_path.ends_with('/') || final_path.ends_with('\\') {
        let remove_last_index = final_path.len() - 1;
        final_path = &final_path[..remove_last_index];
    }

    let date_now = Utc::now().date_naive();
    let date_path = if env::consts::OS == "windows" {
        format!("{final_path}\\{date_now}\\")
    } else {
        format!("{final_path}/{date_now}/")
    };
    // if directory exists, do nothing. else create.
    if fs::create_dir(&date_path).is_ok() {};

    date_path
}

#[must_use]
pub fn get_current_utc_date() -> String {
    Utc::now().date_naive().to_string()
}

pub struct SeriesConfiguration<'a> {
    pub filename: &'a str,
    pub page_url: &'a str,
    pub episode_url: &'a str,
    pub episode_url_offset: u16,
}

#[must_use]
pub fn validate_output_path_ends_correctly(path: &str) -> String {
    if path.ends_with('\\') || path.ends_with('/') {
        return path.to_string();
    }

    let new_path = if env::consts::OS == "windows" {
        format!("{path}\\")
    } else {
        format!("{path}/")
    };

    new_path
}
