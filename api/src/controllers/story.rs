use warp;

use crate::respository::RespositoryTrait;
use crate::{models::story::Story, respository::story::StoryRepository};

use super::{ControllerResult, Response};

pub async fn add_story(
    user_id: String,
    new_story: Story,
    repository: StoryRepository,
) -> ControllerResult<impl warp::Reply> {
    let mut r = repository.clone();
    let story = r
        .create_user_story(new_story, user_id.parse().unwrap())
        .await
        .expect("Error creating a story");
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        failuer_reason: None,
        body: Some(story),
    })))
}

pub async fn get_story(
    story_id: String,
    repository: impl RespositoryTrait<Story>,
) -> ControllerResult<impl warp::Reply> {
    let story = repository
        .clone()
        .get_one(story_id.parse().unwrap())
        .expect("Error getting single story");
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        failuer_reason: None,
        body: Some(story),
    })))
}

pub async fn get_stories(
    repository: impl RespositoryTrait<Story>,
) -> ControllerResult<impl warp::Reply> {
    let stories = repository
        .clone()
        .get_all()
        .expect("Error getting all stories");
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        failuer_reason: None,
        body: Some(stories),
    })))
}

pub async fn delete_story(
    story_id: String,
    repository: impl RespositoryTrait<Story>,
) -> ControllerResult<impl warp::Reply> {
    repository
        .clone()
        .delete(story_id.parse().unwrap())
        .expect("Error deleting a story");
    Ok(Box::new(warp::reply::json(&Response::<Story> {
        status: super::ResponseStatus::SUCCESS,
        failuer_reason: None,
        body: None,
    })))
}

pub async fn update_story(
    story_id: String,
    updated_story_info: Story,
    repository: impl RespositoryTrait<Story>,
) -> ControllerResult<impl warp::Reply> {
    let story = repository
        .clone()
        .update(story_id.parse().unwrap(), updated_story_info)
        .expect("Error updatting a story");
    Ok(Box::new(warp::reply::json(&Response::<Story> {
        status: super::ResponseStatus::SUCCESS,
        failuer_reason: None,
        body: Some(story),
    })))
}

pub async fn get_user_stories(
    user_id: String,
    repository: StoryRepository,
) -> ControllerResult<impl warp::Reply> {
    let mut r = repository.clone();
    let user_stories = r.get_user_stories(user_id.parse().unwrap()).unwrap();
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        failuer_reason: None,
        body: Some(user_stories),
    })))
}
