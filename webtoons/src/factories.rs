use anyhow::{bail, Result};
use rand::Rng;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use tracing::{error, info, warn};

pub struct BlockingReferClient;

impl BlockingReferClient {
    /// # Errors
    ///
    /// Will panic if there it can't connect to URL.
    pub fn get(url: &str) -> Result<reqwest::blocking::Response> {
        let client = Client::builder()
            .user_agent(r"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
            .build()?;

        let mut retries = 5;
        let mut wait = 1;

        // let mut rng = rand::thread_rng();
        // let stop_rng = rng.gen_range(500..=2500);
        // thread::sleep(Duration::from_millis(stop_rng));

        let response = loop {
            info!("Making request to {url}");
            match client
                .get(url)
                .header("referer", "https://www.webtoons.com/")
                .send()
            {
                Err(_) => {
                    warn!("Retrying connection to {url}");
                    if retries > 0 {
                        retries -= 1;
                        thread::sleep(Duration::from_secs(wait));
                        wait *= 2;
                        warn!("Retries left: {retries} ; Wait time: {wait}");
                    } else {
                        error!("Out of retires, failed to connect to {url}");
                        bail!("Cannot connect. Check URL: {url}");
                    }
                }
                Ok(ok) => break ok,
            }
        };

        Ok(response)
    }
}

pub struct BlockingJsonReferClient;

impl BlockingJsonReferClient {
    /// # Errors
    ///
    /// Will panic if there it can't connect to URL.
    pub fn get(url: &str) -> Result<reqwest::blocking::Response> {
        let client = Client::builder()
            .user_agent(r"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/109.0.0.0 Safari/537.36")
            .build()?;
        let mut rng = rand::thread_rng();

        let mut retries = 5;
        let mut wait = 1;

        let stop_rng = rng.gen_range(500..=1500);
        thread::sleep(Duration::from_millis(stop_rng));

        let template: HashMap<String, String> = HashMap::new();

        let response = loop {
            info!("Making request to {url}");
            match client
                .get(url)
                .header("referer", "https://www.webtoons.com/")
                .json(&template)
                .send()
            {
                Err(_) => {
                    warn!("Retrying connection to {url}");
                    if retries > 0 {
                        retries -= 1;
                        thread::sleep(Duration::from_secs(wait));
                        wait *= 2;
                        warn!("Retries left: {retries} ; Wait time: {wait}");
                    } else {
                        error!("Out of retires, failed to connect to {url}");
                        bail!("Cannot connect. Check URL: {url}");
                    }
                }
                Ok(ok) => break ok,
            }
        };

        Ok(response)
    }
}
