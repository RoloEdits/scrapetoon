mod args;

use std::{fs::File, sync::Arc};

use anyhow::Result;
use args::{Args, Command};
use clap::Parser;
use scrapetoon::Stats;
use tokio::sync::Semaphore;
use webtoon::platform::webtoons::{Client, error::EpisodeError};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Args::parse();

    match cli.command {
        Command::Stats {
            path,
            url,
            mut episodes,
        } => {
            let client = Client::new();

            let file = File::create(path)?;
            let mut writer = csv::Writer::from_writer(file);

            let webtoon = client.webtoon_from_url(&url)?;

            let id = webtoon.id();
            let title = webtoon.title().await?;
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
            let genre = webtoon
                .genres()
                .await?
                .first()
                .map(|genre| genre.as_slug())
                .expect("At least one genre must exist");
            let subscribers = webtoon.subscribers().await?;
            let views = webtoon.views().await?;

            while let Some(number) = episodes.next()
                && let Some(episode) = webtoon.episode(number).await?
            {
                eprintln!("Getting stats for episode {number}");

                let likes = episode.likes().await?;
                let (comments, replies) = episode.comments_and_replies().await?;

                let stats = Stats {
                    id,
                    creator: creator.trim_matches(','),
                    title: title.as_str(),
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
        Command::Download {
            path,
            url,
            mut episodes,
        } => {
            let client = Client::new();

            let webtoon = client.webtoon_from_url(&url)?;

            let semaphore = Arc::new(Semaphore::const_new(3));
            let mut handles = Vec::with_capacity(episodes.end());

            while let Some(number) = episodes.next()
                && let Some(episode) = webtoon.episode(number).await?
            {
                let path = path.clone();
                let semaphore = semaphore.clone();

                let handle = tokio::spawn(async move {
                    let _permit = semaphore.acquire_owned().await.unwrap();

                    eprintln!("downloading panels for episode {number}...");
                    match episode.download().await {
                        Ok(panels) => panels.save_single(&path).await.unwrap(),
                        Err(EpisodeError::NotViewable) => {}
                        Err(err) => panic!("{err}"),
                    }
                });

                handles.push(handle);
            }

            for handle in handles {
                handle.await?;
            }
        }
    }

    Ok(())
}
