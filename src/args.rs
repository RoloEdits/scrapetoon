use std::{ops::Range, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Scrapetoon {
    /// Choose the source of data to scrape.
    #[command(subcommand)]
    pub source: Source,
}

#[derive(Subcommand, Debug)]
pub enum Source {
    /// Gets a story's statistics and saves them as a CSV file.
    Stats {
        /// Path to save the output CSV file.
        #[arg(short = 'o', long = "output")]
        path: PathBuf,

        /// URL to the story page.
        ///
        /// Example: `https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95`
        #[arg(short, long)]
        url: String,

        /// Episode or episodes to retrieve statistics for.
        ///
        /// Can be expressed as a single integer (e.g., `150`) or as a range (e.g., `1..100`).
        ///
        /// Ranges are inclusive and can be open-ended:
        ///
        /// - `..200` (up to episode 200)
        ///
        /// - `350..` (starting from episode 350)
        ///
        /// - `..` (all available episodes).
        #[arg(short = 'e', long = "episodes")]
        episodes: String,
    },

    /// Downloads a chapters panels and saves them as an image file.
    ///
    /// Only publicly visible episodes can be downloaded.
    Download {
        /// Path to save the output image files.
        #[arg(short = 'o', long = "output")]
        path: PathBuf,

        /// URL to the story page.
        ///
        /// Example: `https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95`
        #[arg(short, long)]
        url: String,

        /// Episode or episodes to download.
        ///
        /// Can be expressed as a single integer (e.g., `150`) or as a range (e.g., `1..100`).
        ///
        /// Ranges are inclusive and can be open-ended:
        ///
        /// - `..200` (up to and including episode 200)
        ///
        /// - `350..` (starting from episode 350)
        ///
        /// - `..` (all available episodes).
        #[arg(short = 'e', long = "episodes")]
        episodes: String,
    },
}

pub fn parse_range_u16(input: &str) -> Result<Range<u16>, String> {
    if input == ".." {
        return Ok(1..u16::MAX);
    }

    let parts: Vec<&str> = input.split("..").collect();
    match parts.len() {
        1 => {
            let value = parts[0].parse::<u16>().map_err(|_| "Invalid value")?;
            Ok(value..value) // Single value should loop once
        }
        2 => {
            let start = if parts[0].is_empty() {
                1
            } else {
                parts[0].parse::<u16>().map_err(|_| "Invalid start value")?
            };
            let end = if parts[1].is_empty() {
                // NOTE: in theory this could end up excluding the very last chapter if the total number is `u16::MAX`.
                u16::MAX - 1 // Later on 1 is added, so must have this be one less
            } else {
                parts[1].parse::<u16>().map_err(|_| "Invalid end value")?
            };

            if start > end {
                return Err("Start must be less than or equal to end".to_owned());
            }

            Ok(start..end + 1)
        }
        _ => {
            Err("Invalid format. Expected 'start..end', '..end', 'start..', or 'value'".to_owned())
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_single() {
        let range = parse_range_u16("1").unwrap();
        assert_eq!(1..1, range);
    }

    #[test]
    fn should_be_inclusive() {
        let range = parse_range_u16("1..100").unwrap();
        assert_eq!(1..101, range);
    }

    #[test]
    fn should_parse_open_start() {
        let range = parse_range_u16("..100").unwrap();
        assert_eq!(1..101, range);
    }

    #[test]
    fn should_parse_open_end() {
        let range = parse_range_u16("100..").unwrap();
        assert_eq!(100..u16::MAX, range);
    }
}
