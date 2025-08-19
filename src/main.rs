mod args;

use std::fs::File;

use anyhow::{Result, anyhow};
use args::{Scrapetoon, Source};
use clap::Parser;
use serde::Serialize;
use webtoon::platform::webtoons::{Client, errors::EpisodeError};

#[derive(Serialize)]
struct Stats<'a> {
    id: u32,
    creator: &'a str,
    title: &'a str,
    genre: &'a str,
    views: u64,
    subscribers: u32,
    episode: u16,
    likes: u32,
    comments: u32,
    replies: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Scrapetoon::parse();

    let client = Client::new();

    match cli.source {
        Source::Stats {
            path,
            url,
            episodes,
        } => {
            let webtoon = client.webtoon_from_url(&url)?;

            let id = webtoon.id();
            let creator = webtoon
                .creators()
                .await?
                .iter()
                .map(|creator| creator.username())
                .fold(String::new(), |mut builder, name| {
                    builder.push_str(name);
                    builder.push(',');
                    builder
                });
            let title = webtoon.title().await?;
            let genre = webtoon
                .genres()
                .await?
                .first()
                .map(|genre| genre.as_slug())
                .expect("At least one genre must exist");
            let subscribers = webtoon.subscribers().await?;
            let views = webtoon.views().await?;

            let episodes = args::parse_range_u16(&episodes)
                .map_err(|err| anyhow!("failed to parse `{err}` as a valid episode number"))?;

            let file = File::create(path)?;

            let mut writer = csv::Writer::from_writer(file);

            for number in episodes {
                eprintln!("getting stats for episode {number}");
                let Some(episode) = webtoon.episode(number).await? else {
                    continue;
                };

                let likes = episode.likes().await?;
                let (comments, replies) = episode.comments_and_replies().await?;

                let stats = Stats {
                    id,
                    creator: creator.trim_matches(','),
                    title: &title,
                    genre,
                    views,
                    subscribers,
                    episode: number,
                    likes,
                    comments,
                    replies,
                };

                writer.serialize(stats)?;
            }

            writer.flush()?;
        }
        Source::Download {
            path,
            url,
            episodes,
        } => {
            let webtoon = client.webtoon_from_url(&url)?;

            let episodes = args::parse_range_u16(&episodes)
                .map_err(|err| anyhow!("failed to parse `{err}` as a valid episode number"))?;

            for number in episodes {
                eprintln!("downloading panels for episode {number}");
                let Some(episode) = webtoon.episode(number).await? else {
                    continue;
                };

                match episode.download().await {
                    Ok(panels) => panels.save_single(&path).await?,
                    Err(EpisodeError::NotViewable) => {}
                    Err(err) => return Err(err.into()),
                }
            }
        }
    }

    Ok(())
}
