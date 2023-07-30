use std::sync::Arc;

use diesel;
use diesel::prelude::*;

use super::{DbPool, RespositoryResult, RespositoryTrait};
use crate::{
    models::{
        story::Story,
        user_story::{UserStory, UserWithStories},
    },
    services::notifcations::StoryNoticationService,
};
#[derive(Clone)]
pub struct StoryRepository(pub(crate) DbPool);

impl RespositoryTrait<Story> for StoryRepository {
    fn create(&mut self, story: Story) -> RespositoryResult<Story> {
        use crate::schema::story;
        diesel::insert_into(story::table)
            .values(&story)
            .get_result::<Story>(&mut self.0.get().expect("Error getting connection"))
    }

    fn get_all(&mut self) -> RespositoryResult<Vec<Story>> {
        use crate::schema::story::dsl::*;
        let stories = story
            .load::<Story>(&mut self.0.get().expect(""))
            .expect("Error getting all stories");
        Ok(stories)
    }

    fn get_one(&mut self, model_id: i32) -> RespositoryResult<Story> {
        use crate::schema::story::dsl::*;
        let story_item = story
            .find(model_id)
            .first::<Story>(&mut self.0.get().expect("Error getting connection"))
            .expect("Error getting single story");
        Ok(story_item)
    }

    fn update(&mut self, model_id: i32, model: Story) -> RespositoryResult<Story> {
        use crate::schema::story::dsl::*;
        diesel::update(story.find(model_id))
            .set(&model)
            .get_result::<Story>(&mut self.0.get().expect("Error getting connection"))
    }

    fn delete(&mut self, model_id: i32) -> RespositoryResult<()> {
        use crate::schema::story::dsl::*;
        diesel::delete(story.find(model_id))
            .execute(&mut self.0.get().expect("Error getting connection"))
            .expect("Error deleting a story");
        Ok(())
    }
}

impl StoryRepository {
    pub async fn create_user_story(
        &mut self,
        story: Story,
        user_id: i32,
    ) -> RespositoryResult<Story> {
        use crate::schema::story;
        let mut conn = self.0.get().expect("Error getting connection");
        let new_story = diesel::insert_into(story::table)
            .values(&story)
            .get_result::<Story>(&mut conn)
            .unwrap();
        use crate::schema::user_story;
        let user_story_item = UserStory {
            id: None,
            user_id,
            story_id: new_story.id().unwrap(),
        };
        diesel::insert_into(user_story::table)
            .values(user_story_item)
            .execute(&mut conn)
            .expect("Error creating user story join table row");

        StoryNoticationService::notify(user_id).await;
        Ok(new_story)
    }
    pub fn get_user_stories(&mut self, story_user_id: i32) -> RespositoryResult<UserWithStories> {
        use crate::schema::user_story::dsl::*;
        let stories = Arc::new(
            user_story
                .inner_join(crate::schema::story::table)
                .filter(crate::schema::user_story::user_id.eq(story_user_id))
                .select(Story::as_select()),
        )
        .load(&mut self.0.get().expect("Error getting connection"))
        .expect("Error getting the stories for a user");

        Ok(UserWithStories::new(story_user_id, stories))
    }
}
