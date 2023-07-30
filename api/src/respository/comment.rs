use std::sync::Arc;

use diesel;
use diesel::prelude::*;

use super::{DbPool, RespositoryResult, RespositoryTrait};
use crate::models::{
    comment::{Comment, StoryWithComments},
    story::Story,
};
#[derive(Clone)]
pub struct CommentRepository(pub(crate) DbPool);

unsafe impl Send for CommentRepository {}

impl RespositoryTrait<Comment> for CommentRepository {
    fn create(&mut self, comment: Comment) -> RespositoryResult<Comment> {
        use crate::schema::comment;
        diesel::insert_into(comment::table)
            .values(&comment)
            .get_result::<Comment>(&mut self.0.get().expect("Error getting connection"))
    }

    fn get_all(&mut self) -> RespositoryResult<Vec<Comment>> {
        use crate::schema::comment::dsl::*;
        let comments = comment
            .load::<Comment>(&mut self.0.get().expect(""))
            .expect("Error getting all comments");
        Ok(comments)
    }

    fn get_one(&mut self, model_id: i32) -> RespositoryResult<Comment> {
        use crate::schema::comment::dsl::*;
        let comment_item = comment
            .find(model_id)
            .first::<Comment>(&mut self.0.get().expect("Error getting connection"))
            .expect("Error getting single comment");
        Ok(comment_item)
    }

    fn update(&mut self, model_id: i32, model: Comment) -> RespositoryResult<Comment> {
        use crate::schema::comment::dsl::*;
        diesel::update(comment.find(model_id))
            .set(&model)
            .get_result::<Comment>(&mut self.0.get().expect("Error getting connection"))
    }

    fn delete(&mut self, model_id: i32) -> RespositoryResult<()> {
        use crate::schema::comment::dsl::*;
        diesel::delete(comment.find(model_id))
            .execute(&mut self.0.get().expect("Error getting connection"))
            .expect("Error deleting a comment");
        Ok(())
    }
}

impl CommentRepository {
    pub fn get_all_story_comments(
        &mut self,
        model_id: i32,
    ) -> RespositoryResult<StoryWithComments> {
        use crate::schema::comment::dsl::*;
        let results: Vec<(Comment, Story)> = Arc::new(
            comment
                .inner_join(crate::schema::story::table)
                .filter(crate::schema::story::id.eq(model_id))
                .select((Comment::as_select(), Story::as_select())),
        )
        .load::<(Comment, Story)>(&mut self.0.get().expect("Error getting connection"))
        .expect("Error getting a comments for a story");

        let story_with_comments = StoryWithComments::new(results);
        Ok(story_with_comments)
    }
}
