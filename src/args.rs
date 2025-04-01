use std::{ops::Range, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Scrapetoon {
    /// Which source of data you wish to scrape
    #[command(subcommand)]
    pub source: Source,
}

#[derive(Subcommand, Debug)]
pub enum Source {
    /// Scrapes a stories page
    Stats {
        /// Path to save the output file
        #[arg(short = 'o', long = "output")]
        path: PathBuf,

        /// URL to the Story Page
        #[arg(short, long)]
        url: String,

        /// Episode or Episodes to get.
        ///
        /// Can be expressed as a single integer, `150`, or as a range, `1..100`.
        ///
        /// Ranges are inclusive and can be open ended: `..200`, `350..`.
        #[arg(short = 'e', long = "episodes")]
        episodes: String,
    },

    /// Scrapes chapters to download as an image file
    Panels {
        /// Path to save the output image files
        #[arg(short = 'o', long = "output")]
        path: PathBuf,

        /// URL to the Story Page
        #[arg(short, long)]
        url: String,

        /// Episode or Episodes to get.
        ///
        /// Can be expressed as a single integer, `150`, or as a range, `1..100`.
        ///
        /// Ranges are inclusive and can be open ended: `..200`, `350..`.
        #[arg(short = 'e', long = "episodes")]
        episodes: String,
    },
}

pub fn parse_range_u16(input: &str) -> Result<Range<u16>, &'static str> {
    let parts: Vec<&str> = input.split("..").collect();
    match parts.len() {
        1 => {
            let value = parts[0].parse::<u16>().map_err(|_| "Invalid value")?;
            Ok(value..value) // Single value should create an empty range that runs once in a loop
        }
        2 => {
            let start = parts[0].parse::<u16>().map_err(|_| "Invalid start value")?;
            let end = parts[1].parse::<u16>().map_err(|_| "Invalid end value")?;

            if start > end {
                return Err("Start must be less than or equal to end");
            }

            Ok(start..end + 1)
        }
        _ => Err("Invalid format. Expected 'start..end' or 'value'"),
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
}
