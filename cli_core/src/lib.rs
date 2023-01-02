use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

pub struct ProgressBarFactory {}

impl ProgressBarFactory {
    #[must_use]
    /// # Panics
    ///
    /// Will panic if unable to unwrap bar style. In theory should never panic unless library API changes.
    pub fn get_bar(count: u16) -> ProgressBar {
        let bar = ProgressBar::new(u64::from(count));

        bar.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] [{wide_bar:.white/black} {pos}/{len}",
            )
            .unwrap(),
        );
        bar
    }
}

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct StoryCliArgs {
    /// The starting chapter. The lowest number between Start and End.
    #[arg(short, long)]
    pub start: u16,

    /// The ending chapter. The highest number between Start and End.
    #[arg(short, long)]
    pub end: u16,

    /// The amount of pages that would minimally encompass the given Start and End.
    #[arg(short, long)]
    pub pages: u16,

    /// The output path to save the output csv file.
    #[arg(short, long)]
    pub output: String,
}
