use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CliArgs {
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