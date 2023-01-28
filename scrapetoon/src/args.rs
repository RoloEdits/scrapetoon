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
    Story {
        /// Path to save the output file
        #[arg(short = 'o', long = "output")]
        path: String,

        /// URL to the Story Page
        #[arg(short, long)]
        url: String,

        /// The earliest chapter
        #[arg(short = 's', long = "start")]
        start: u16,

        /// The latest chapter
        #[arg(short = 'e', long = "end")]
        end: u16,

        // TODO: Pages isn't needed if the story is completed. Right now its value must always be given. Would be ideal if this can be gotten without use input.
        /// The largest page number of the stories chapter lists
        #[arg(short = 'p', long = "pages")]
        pages: u16,

        /// The latest chapter
        #[arg(short = 'c', long = "completed")]
        completed: bool,
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
