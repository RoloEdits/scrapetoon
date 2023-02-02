use crate::story::chapter::comments::models::UserComment;
use crate::story::models::Story;
use serde::Serialize;
use tracing::info;

#[derive(Serialize, Debug, Default, Clone)]
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
    pub username: Option<String>,
    pub id: Option<String>,
    pub country: Option<String>,
    pub profile_type: Option<String>,
    pub auth_provider: Option<String>,
    pub upvotes: Option<u32>,
    pub downvotes: Option<u32>,
    pub comment_replies: Option<u32>,
    pub post_date: Option<String>,
    pub contents: Option<String>,
    pub timestamp: String,
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

        // TODO: Decouple comments in prep for option to have them scraped or not.
        for chapter in self.chapters {
            let chapter_record = StoryRecord {
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
                username: None,
                id: None,
                country: None,
                profile_type: None,
                auth_provider: None,
                upvotes: None,
                downvotes: None,
                comment_replies: None,
                post_date: None,
                contents: None,
                timestamp: chapter.timestamp.clone(),
            };

            if let Some(comments) = chapter.user_comments {
                for comment in comments {
                    let id = get_valid_user_id(&comment);

                    let comment_record = StoryRecord {
                        username: Option::from(comment.username),
                        id,
                        country: Option::from(comment.country),
                        profile_type: Option::from(comment.profile_type),
                        auth_provider: Option::from(comment.auth_provider),
                        upvotes: Option::from(comment.upvotes),
                        downvotes: Option::from(comment.downvotes),
                        comment_replies: Option::from(comment.replies),
                        post_date: Option::from(comment.post_date),
                        contents: Option::from(comment.contents.replace('\n', " ")),
                        ..chapter_record.clone()
                    };

                    record.push(comment_record);
                }
            } else {
                record.push(chapter_record);
            }
        }

        info!("Finished making Story Record");
        record
    }
}

fn get_valid_user_id(comment: &UserComment) -> Option<String> {
    if comment.id_no.is_some() {
        return comment.id_no.clone();
    }

    if comment.user_id_no.is_some() {
        return comment.user_id_no.clone();
    }

    if comment.profile_user_id.is_some() {
        return comment.profile_user_id.clone();
    }

    None
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

        // let user_comments = vec![UserComment {
        //     username: String::new(),
        //     replies: 0,
        //     upvotes: 0,
        //     downvotes: 0,
        //     contents: String::new(),
        //     profile_type: String::new(),
        //     auth_provider: String::new(),
        //     country: String::new(),
        //     post_date: String::new(),
        //     id_no: Some(String::new()),
        //     user_id_no: None,
        //     profile_user_id: None,
        // }];

        let chapters: Vec<_> = vec![Chapter::<String> {
            number: 1,
            likes: 4_000,
            length: Some(2_000),
            comments: 1_000,
            replies: 2_000,
            season: None,
            season_chapter: None,
            arc: None,
            user_comments: None,
            published: None,
            timestamp: String::new(),
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
