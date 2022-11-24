use chrono::Utc;
use core::time;
use reqwest::{blocking::Response, Error};
use std::{env, fs, thread};

pub struct ResponseFactory {}

impl ResponseFactory {
    pub fn get(url: &str) -> Result<Response, Error> {
        let mut retries = 5;
        let mut wait = 1;

        let response: Response = loop {
            match reqwest::blocking::get(url) {
                Err(_) => {
                    if retries > 0 {
                        retries -= 1;
                        thread::sleep(time::Duration::from_secs(wait));
                        wait *= 2;
                    } else {
                        panic!("Cannot connect. Check URL: {}", url)
                    }
                }
                Ok(ok) => break ok,
            }
        };

        return Ok(response);
    }
}

pub fn create_date_folder(filepath: &str) -> String {
    let mut final_path = filepath;

    // Equalizes all paths so that an operation to add slashes can be done without worry of doubling up.
    if final_path.ends_with('/') || final_path.ends_with('\\') {
        let remove_last_index = final_path.len() - 1;

        final_path = &final_path[..remove_last_index]
    }

    let date_now = Utc::now().date_naive();
    let date_path = if env::consts::OS == "windows" {
        format!("{}\\{}\\", final_path, date_now)
    } else {
        format!("{}/{}/", final_path, date_now)
    };
    // if directory exists, do nothing. else create.
    if let Ok(ok) = fs::create_dir(&date_path) {
        ok
    };

    date_path
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {

//     }
// }
