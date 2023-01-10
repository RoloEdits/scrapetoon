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
    /// Scrapes Daily Schedule
    Daily {
        /// Path to save the output file
        #[arg(short = 'o', long = "output")]
        path: String,
    },

    /// Scrapes a stories page
    Page {
        /// Path to save the output file
        #[arg(short = 'o', long = "output")]
        path: String,

        /// URL to the Story Page
        #[arg(short, long)]
        url: String,

        /// The final page, where the story starts
        #[arg(short = 'e', long = "end")]
        end: u16,
    },

    /// Scrapes chapters to download as an image file
    Panels {
        /// Path to save the output image files
        #[arg(short = 'o', long = "output")]
        path: String,

        /// URL to the Story Page
        #[arg(short, long)]
        url: String,

        /// The earliest of the chapters to download
        #[arg(short = 's', long = "start")]
        start: u16,

        /// The latest chapter to download
        #[arg(short = 'e', long = "end")]
        end: u16,
    },
}
