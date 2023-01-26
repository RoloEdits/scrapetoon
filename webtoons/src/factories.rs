use anyhow::{bail, Result};
use core::time;
use rand::Rng;
use reqwest::blocking::Client;
use reqwest::Response;
use std::thread;
use std::time::Duration;

pub struct ResponseFactory;

impl ResponseFactory {
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
                        thread::sleep(Duration::from_secs(wait + wait_rng));
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

pub struct BlockingResponseFactory;

impl BlockingResponseFactory {
    /// # Errors
    ///
    /// Will panic if there it can't connect to URL.
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
                        thread::sleep(Duration::from_secs(wait + wait_rng));
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

pub struct BlockingReferClientFactory;

impl BlockingReferClientFactory {
    /// # Errors
    ///
    /// Will panic if there it can't connect to URL.
    pub fn get(url: &str) -> Result<reqwest::blocking::Response> {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
            .build()?;
        // let mut rng = rand::thread_rng();

        let mut retries = 5;
        let mut wait = 1;
        //
        // let stop_rng = rng.gen_range(1..3);
        //
        // thread::sleep(Duration::from_secs(stop_rng));

        let response = loop {
            match client
                .get(url)
                .header("referer", "https://www.webtoons.com/")
                .send()
            {
                Err(_) => {
                    if retries > 0 {
                        retries -= 1;
                        thread::sleep(Duration::from_secs(wait));
                        wait *= 2;
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
