use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct LineArgs {
    /// Which source of data you wish to scrape
    #[command(subcommand)]
    pub source: SourceData,
}

#[derive(Subcommand, Debug)]
pub enum SourceData {
    /// Scrapes Daily Schedule
    Daily {
        /// Path to save the output file
        #[arg(short = 'o', long = "output")]
         output: String,
    },

    /// Scrapes a stories page
    Story {
        /// Path to save the output file
        #[arg(short = 'o', long = "output")]
        output: String,

        /// URL to the Story Page
        #[arg(short, long)]
        url: String,

        /// The final page, where the story starts
        #[arg(short = 'e', long = "end-page")]
        end: u16,
    }
}