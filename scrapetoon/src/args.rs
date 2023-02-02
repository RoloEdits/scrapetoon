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

        /// Include the `top` comments in the output
        #[arg(long = "top-comments")]
        top_comments: bool,

        /// Include all comments in the output
        #[arg(long = "all-comments")]
        all_comments: bool,

        /// The number of threads program will use. Each one correlating to how many chapters will be scraped in parallel. USE WITH CAUTION.
        /// Trying to get too many chapters at once can cause timeout issues. Defaults to `4` threads. Setting to `0` will use all available threads.
        #[arg(short = 't', long = "threads")]
        threads: Option<usize>,
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

        /// The number of threads program will use. Each one correlating to how many chapters will be scraped in parallel. USE WITH CAUTION.
        /// Trying to get too many chapters at once can cause timeout issues. Defaults to `6` threads. Setting to `0` will use all available threads.
        #[arg(short = 't', long = "threads")]
        threads: Option<usize>,
    },
}
