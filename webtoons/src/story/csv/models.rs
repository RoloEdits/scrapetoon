use crate::story::models::Story;
use serde::Serialize;
use tracing::info;

// TODO: Think about adding an optional `custom` field that can be any other think the implementer wants that's not already covered

#[derive(Serialize, Debug)]
pub struct StoryRecord<T> {
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
    pub custom: Option<T>,
    pub length: Option<u32>,
    pub comments: u32,
    pub total_comments: u32,
    pub replies: u32,
    pub total_replies: u32,
    pub likes: u32,
    pub total_likes: u32,
    pub published: Option<String>,
    pub username: String,
    pub id: Option<String>,
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

pub trait IntoStoryRecord<T: Clone + Send> {
    fn into_record(self) -> Vec<StoryRecord<T>>;
}

impl<T: Clone + Send> IntoStoryRecord<T> for Story<T> {
    fn into_record(self) -> Vec<StoryRecord<T>> {
        info!("Making Story Record");
        let mut record: Vec<StoryRecord<T>> = Vec::new();

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
                    custom: chapter.custom.clone(),
                    length: chapter.length,
                    comments: chapter.comments,
                    total_comments,
                    replies: chapter.replies,
                    total_replies,
                    likes: chapter.likes,
                    total_likes,
                    published: chapter.published.clone(),
                    username: comment.username,
                    id: comment.id,
                    country: comment.country,
                    profile_type: comment.profile_type,
                    auth_provider: comment.auth_provider,
                    upvotes: comment.upvotes,
                    downvotes: comment.downvotes,
                    comment_replies: comment.replies,
                    post_date: comment.post_date,
                    contents: comment.contents,
                    scrape_date: chapter.scraped.clone(),
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

impl<T: Clone + Send> SumComments for Story<T> {
    fn sum_comments(&self) -> u32 {
        self.chapters
            .iter()
            .fold(0, |acc, chapter| acc + chapter.comments)
    }
}

trait SumReplies {
    fn sum_replies(&self) -> u32;
}

impl<T: Clone + Send> SumReplies for Story<T> {
    fn sum_replies(&self) -> u32 {
        self.chapters
            .iter()
            .fold(0, |acc, chapter| acc + chapter.replies)
    }
}

trait SumLikes {
    fn sum_likes(&self) -> u32;
}

impl<T: Clone + Send> SumLikes for Story<T> {
    fn sum_likes(&self) -> u32 {
        self.chapters
            .iter()
            .fold(0, |acc, chapter| acc + chapter.likes)
    }
}

#[cfg(test)]
mod story_csv_tests {
    use super::*;
    use crate::story::chapter::comments::models::UserComment;
    use crate::story::chapter::models::Chapter;
    use crate::story::models::StoryPage;

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
            username: String::new(),
            replies: 0,
            upvotes: 0,
            downvotes: 0,
            contents: String::new(),
            profile_type: String::new(),
            auth_provider: String::new(),
            country: String::new(),
            post_date: String::new(),
            id: Some(String::new()),
        }];

        let chapters: Vec<_> = vec![Chapter::<String> {
            number: 1,
            likes: 4_000,
            length: Some(2_000),
            comments: 1_000,
            replies: 2_000,
            season: None,
            season_chapter: None,
            arc: None,
            user_comments,
            published: None,
            scraped: String::new(),
            custom: None,
        }];

        let test_against = chapters.first().unwrap().comments;

        let story = Story::<String> {
            story_page,
            chapters,
        };

        let record = story.into_record();

        assert_eq!(record.len(), 1);

        let record_comments = record.first().unwrap().comments;

        assert_eq!(record_comments, test_against);
    }
}
