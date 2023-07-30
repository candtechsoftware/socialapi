use warp;

use crate::models::comment::Comment;
use crate::respository::comment::CommentRepository;
use crate::respository::RespositoryTrait;

use super::{ControllerResult, Response};

pub async fn add_comment(
    new_comment: Comment,
    repository: impl RespositoryTrait<Comment>,
) -> ControllerResult<impl warp::Reply> {
    let comment = repository
        .clone()
        .create(new_comment)
        .expect("Error creating a comment");
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        body: Some(comment),
    })))
}

pub async fn get_comment(
    comment_id: String,
    repository: impl RespositoryTrait<Comment>,
) -> ControllerResult<impl warp::Reply> {
    let comment = repository
        .clone()
        .get_one(comment_id.parse().unwrap())
        .expect("Error getting single comment");
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        body: Some(comment),
    })))
}

pub async fn get_comments(
    repository: impl RespositoryTrait<Comment>,
) -> ControllerResult<impl warp::Reply> {
    let comments = repository
        .clone()
        .get_all()
        .expect("Error getting all comments");
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        body: Some(comments),
    })))
}

pub async fn delete_comment(
    comment_id: String,
    repository: impl RespositoryTrait<Comment>,
) -> ControllerResult<impl warp::Reply> {
    repository
        .clone()
        .delete(comment_id.parse().unwrap())
        .expect("Error deleting a comment");
    Ok(Box::new(warp::reply::json(&Response::<Comment> {
        status: super::ResponseStatus::SUCCESS,
        body: None,
    })))
}

pub async fn update_comment(
    comment_id: String,
    updated_comment_info: Comment,
    repository: impl RespositoryTrait<Comment>,
) -> ControllerResult<impl warp::Reply> {
    let comment = repository
        .clone()
        .update(comment_id.parse().unwrap(), updated_comment_info)
        .expect("Error updatting a comment");
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        body: Some(comment),
    })))
}

pub async fn get_all_story_comments(
    story_id: String,
    repository: CommentRepository,
) -> ControllerResult<impl warp::Reply> {
    let mut r = repository.clone();
    let story_with_comments = r.get_all_story_comments(story_id.parse().unwrap()).unwrap();
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        body: Some(story_with_comments),
    })))
}
