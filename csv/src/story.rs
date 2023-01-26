use crate::CsvWrite;
use anyhow::{Context, Result};
use serde::Serialize;
use std::path::Path;
use tracing::info;
use webtoons::story::models::Story;
use webtoons::utils;

#[derive(Serialize, Debug)]
pub struct StoryRecord {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub status: String,
    pub release_day: String,
    pub views: u64,
    pub subscribers: u32,
    pub rating: f32,
    pub chapter: u16,
    pub season: Option<u8>,
    pub season_chapter: Option<u16>,
    pub arc: Option<String>,
    pub length: u32,
    pub comments: u32,
    pub total_comments: u32,
    pub replies: u32,
    pub total_replies: u32,
    pub likes: u32,
    pub total_likes: u32,
    pub published: Option<String>,
    pub username: String,
    pub country: String,
    pub profile_type: String,
    pub auth_provider: String,
    pub upvotes: u32,
    pub downvotes: u32,
    pub comment_replies: u32,
    pub post_date: String,
    pub contents: String,
    pub scrape_date: String,
}

impl CsvWrite for Vec<StoryRecord> {
    fn write(self, path: &Path, filename: &str) -> Result<()> {
        info!("Writing to csv");
        let csv_name = format!("{filename}.csv");
        let mut writer = csv::Writer::from_path(path.join(csv_name))
            .context("File is open in another application")?;

        for data in self {
            info!("Writing row");
            writer.serialize(data).context("Couldn't write to file.")?;
        }

        writer.flush().context("Couldn't flush buffer.")?;

        info!("Flushed buffer");

        Ok(())
    }
}

pub trait IntoStoryRecord {
    fn into_record(self) -> Vec<StoryRecord>;
}

impl IntoStoryRecord for Story {
    fn into_record(self) -> Vec<StoryRecord> {
        info!("Making Story Record");
        let mut record: Vec<StoryRecord> = Vec::new();

        let total_comments = self.sum_comments();
        let total_replies = self.sum_replies();
        let total_likes = self.sum_likes();

        let title = self.story_page.title;
        let author = self.story_page.author;
        let genre = self.story_page.genre;
        let status = self.story_page.status;
        let release_day = self.story_page.release_day;
        let views = self.story_page.views;
        let subscribers = self.story_page.subscribers;
        let rating = self.story_page.rating;

        // TODO: Look into putting this per chapter so each chapters date is logged
        let utc = utils::get_current_utc_date_verbose();

        for chapter in self.chapters {
            for comment in chapter.user_comments {
                let converted = StoryRecord {
                    title: title.clone(),
                    author: author.clone(),
                    genre: genre.clone(),
                    status: status.clone(),
                    release_day: release_day.clone(),
                    views,
                    subscribers,
                    rating,
                    chapter: chapter.number,
                    season: chapter.season,
                    season_chapter: chapter.season_chapter,
                    arc: chapter.arc.clone(),
                    length: chapter.length,
                    comments: chapter.comments,
                    total_comments,
                    replies: chapter.replies,
                    total_replies,
                    likes: chapter.likes,
                    total_likes,
                    published: chapter.published.clone(),
                    username: comment.username,
                    country: comment.country,
                    profile_type: comment.profile_type,
                    auth_provider: comment.auth_provider,
                    upvotes: comment.upvotes,
                    downvotes: comment.downvotes,
                    comment_replies: comment.replies,
                    post_date: comment.post_date,
                    contents: comment.contents,
                    scrape_date: utc.clone(),
                };

                record.push(converted);
            }
        }

        info!("Finished making Story Record");
        record
    }
}

trait SumComments {
    fn sum_comments(&self) -> u32;
}

impl SumComments for Story {
    fn sum_comments(&self) -> u32 {
        self.chapters
            .iter()
            .fold(0, |acc, chapter| acc + chapter.comments)
    }
}

trait SumReplies {
    fn sum_replies(&self) -> u32;
}

impl SumReplies for Story {
    fn sum_replies(&self) -> u32 {
        self.chapters
            .iter()
            .fold(0, |acc, chapter| acc + chapter.replies)
    }
}

trait SumLikes {
    fn sum_likes(&self) -> u32;
}

impl SumLikes for Story {
    fn sum_likes(&self) -> u32 {
        self.chapters
            .iter()
            .fold(0, |acc, chapter| acc + chapter.likes)
    }
}

#[cfg(test)]
mod story_csv_tests {
    use super::*;
    use webtoons::story::chapter::comments::models::UserComment;
    use webtoons::story::chapter::models::Chapter;
    use webtoons::story::models::StoryPage;

    #[test]
    fn should_convert_to_story_record_one_to_one() {
        let story_page = StoryPage {
            title: "Tower of God".to_string(),
            author: "SIU".to_string(),
            genre: "Fantasy".to_string(),
            status: "Hiatus".to_string(),
            release_day: "Sunday".to_string(),
            views: 0,
            subscribers: 0,
            rating: 0.0,
        };

        let user_comments = vec![UserComment {
            username: "".to_string(),
            replies: 0,
            upvotes: 0,
            downvotes: 0,
            contents: "".to_string(),
            profile_type: "".to_string(),
            auth_provider: "".to_string(),
            country: "".to_string(),
            post_date: "".to_string(),
        }];

        let chapters = vec![Chapter {
            number: 1,
            likes: 4_000,
            length: 2_000,
            comments: 1_000,
            replies: 2_000,
            season: None,
            season_chapter: None,
            arc: None,
            user_comments,
            published: None,
        }];

        let test_against = chapters.first().unwrap().comments;

        let story = Story {
            story_page,
            chapters,
        };

        let record = story.into_record();

        assert_eq!(record.len(), 1);

        let record_comments = record.first().unwrap().comments;

        assert_eq!(record_comments, test_against);
    }
}
