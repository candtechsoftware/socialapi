use diesel;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::comment;

use super::{story::Story, user::UserAccount};

#[derive(
    AsChangeset,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Insertable,
    Selectable,
    Associations,
    PartialEq,
    Debug,
)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Story, foreign_key = user_id))]
#[diesel(belongs_to(UserAccount, foreign_key = story_id))]
#[diesel(table_name = comment)]
pub struct Comment {
    #[diesel(deserialize_as = i32)]
    id: Option<i32>,
    data: String,
    user_id: i32,
    story_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct StoryWithComments {
    story: Story,
    comments: Vec<Comment>,
}

impl StoryWithComments {
    pub fn new(comment_story: Vec<(Comment, Story)>) -> Self {
        let story = comment_story[0].1.clone();
        let mut comments = Vec::new();
        for (c, _) in comment_story {
            comments.push(c);
        }
        Self { story, comments }
    }
}
