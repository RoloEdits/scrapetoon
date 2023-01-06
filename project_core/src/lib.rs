use chrono::Utc;
use core::time;
use reqwest::{Error, Response};
use std::path::Path;
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
pub fn create_date_folder(filepath: &str) -> String {
    let path = Path::new(filepath);

    let date_now = get_current_utc_date();

    let date_path = path
        .join(date_now)
        .into_os_string()
        .into_string()
        .expect("date_path");

    fs::create_dir(&date_path).expect("Create date folder");

    date_path
}
// TODO: Handle end of path demarcation
#[must_use]
pub fn path_enforcer(filepath: &str) -> String {
    let path = Path::new(filepath);

    if !path.try_exists().expect("Check if given path exists") {
        fs::create_dir(path).expect("Create given path so it's existence is enforced");
    }

    filepath.to_string()
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     // TODO: Need to mock
//
//     #[test]
//     fn should_create_date_folder() {
//         let given_path = r"D:\temp";
//
//         let date = get_current_utc_date();
//
//         let result = create_date_folder(given_path);
//
//         let test = format!("{given_path}\\{date}");
//
//         assert_eq!(result, test);
//     }
//
//     #[test]
//     fn should_create_valid_folder() {
//         let given_path = r"D:\temp\temp";
//
//         let result = path_enforcer(given_path);
//
//         assert_eq!(result, given_path);
//     }
// }
