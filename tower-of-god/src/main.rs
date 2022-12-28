use args::CliArgs;
use clap::*;
use project_core::SeriesConfiguration;

mod args;
mod csv;

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    const CONFIG: SeriesConfiguration = SeriesConfiguration {
        // eg. 'tower-of-god.csv' is the output
        filename: "tower-of-god",

        // Make sure to only put up to the 'title_no=' and then whatever the ID is.
        // Remove the &page={}' if needed.  
        page_url: "https://www.webtoons.com/en/fantasy/tower-of-god/list?title_no=95",

        // Make sure that the URL ends with the equals. The numbers after point to the specific episode page but we dont need that.
        episode_url: "https://www.webtoons.com/en/fantasy/tower-of-god/season-3-ep-133/viewer?title_no=95&episode_no=",

        // Where you would put the URL 'episode_no=' offset.
        // If the chapter number and the URL number dont match, put the difference here.
        // eg. episode 550 of ToG is really 551 in the URL number, so you put 1 here.
        // This is most common when there is a chapter 0.
        // The goal is to align the chapter number seen on the list page with the input values used in the CLI run.
        // This makes it much easier to use for everyone and gives expected results.
        episode_url_offset: 1,
    };

    // If there is any need to skip a number, this is where you put it.
    let to_skip =  |chapter: u16| -> bool {

        // Add the url number that you need to skip here and return true.
        // For first test run this should always return false. There may not be a need to skip anything.
        match chapter {
            // The URl no=221 for chapter 221 is a 404. No=222 is where #221 is.
            221 => true,
            _ => false
        }
    };

    let parsed_chapters = tower_of_god::parse_chapters(args.start, args.end, args.pages, &CONFIG, to_skip).await;

    csv::write(&args.output, &parsed_chapters, CONFIG.filename);
}

