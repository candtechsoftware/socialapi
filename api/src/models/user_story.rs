use diesel;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::user_story;

use super::{story::Story, user::UserAccount};

#[derive(
    AsChangeset,
    Insertable,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Selectable,
    Associations,
    PartialEq,
    Debug,
)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Story, foreign_key = user_id))]
#[diesel(belongs_to(UserAccount, foreign_key = story_id))]
#[diesel(table_name = user_story)]
pub struct UserStory {
    #[diesel(deserialize_as = i32)]
    pub id: Option<i32>,
    pub user_id: i32,
    pub story_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct UserWithStories {
    user_id: i32,
    stories: Vec<Story>,
}

impl UserWithStories {
    pub fn new(user_id: i32, stories: Vec<Story>) -> Self {
        Self { user_id, stories }
    }
}
