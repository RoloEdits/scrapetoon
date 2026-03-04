use std::{ops::Range, path::PathBuf, str::FromStr};

use anyhow::{Context, bail};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
    /// Choose the source of data to scrape.
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
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
        episodes: Episodes,
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
        episodes: Episodes,
    },
}

#[derive(Debug, Clone)]
pub struct Episodes(Range<u16>);

impl Episodes {
    #[inline]
    pub const fn end(&self) -> usize {
        self.0.end as usize
    }
}

impl FromStr for Episodes {
    type Err = anyhow::Error;

    #[inline]
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s == ".." {
            return Ok(Self(1..u16::MAX));
        }

        let parts: Vec<&str> = s.split("..").collect();
        match parts.len() {
            1 => {
                let value = parts[0].parse::<u16>().context("Invalid value")?;
                Ok(Self(value..value + 1)) // Single value should loop once
            }
            2 => {
                let start = if parts[0].is_empty() {
                    1
                } else {
                    parts[0].parse::<u16>().context("Invalid start value")?
                };
                let end = if parts[1].is_empty() {
                    // NOTE: in theory this could end up excluding the very last chapter if the total number is `u16::MAX`.
                    u16::MAX - 1 // Later on 1 is added, so must have this be one less
                } else {
                    parts[1].parse::<u16>().context("Invalid end value")?
                };

                if start > end {
                    bail!("Start must be less than or equal to end");
                }

                Ok(Self(start..end + 1))
            }
            _ => {
                bail!("Invalid format. Expected 'start..end', '..end', 'start..', or 'value'")
            }
        }
    }
}

impl Iterator for Episodes {
    type Item = u16;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_single() {
        let Episodes(range): Episodes = "1".parse().unwrap();
        assert_eq!(1..2, range);
    }

    #[test]
    fn should_be_inclusive() {
        let Episodes(range): Episodes = "1..100".parse().unwrap();
        assert_eq!(1..101, range);
    }

    #[test]
    fn should_parse_open_start() {
        let Episodes(range): Episodes = "..100".parse().unwrap();
        assert_eq!(1..101, range);
    }

    #[test]
    fn should_parse_open_end() {
        let Episodes(range): Episodes = "100..".parse().unwrap();
        assert_eq!(100..u16::MAX, range);
    }
}
