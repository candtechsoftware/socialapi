use std::borrow::BorrowMut;

use warp::Filter;

use crate::{
    controllers,
    models::comment::Comment,
    respository::{comment::CommentRepository, DbPool},
};

use super::{json_body, with_repository};

pub fn create_comment_routes(
    dbpool: DbPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let comment_repository = CommentRepository { 0: dbpool.clone() };
    get_comment(comment_repository.clone().borrow_mut())
        .or(update_comment(comment_repository.clone().borrow_mut()))
        .or(delete_comment(comment_repository.clone().borrow_mut()))
        .or(create_comment(comment_repository.clone().borrow_mut()))
        .or(get_comments(comment_repository.clone().borrow_mut()))
        .or(get_all_story_comments(comment_repository.clone()))
}

/// method: GET
/// path: /comment/{id}
fn get_comment(
    repo: &mut CommentRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("comment" / String)
        .and(warp::get())
        .and(with_repository(repo.clone()))
        .and_then(controllers::comment::get_comment)
}

/// method: PUT
/// path: /comment/{id}
fn update_comment(
    repo: &mut CommentRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("comment" / String)
        .and(warp::put())
        .and(json_body::<Comment>())
        .and(with_repository(repo.clone()))
        .and_then(controllers::comment::update_comment)
}

/// method: DELETE
/// path: /comment/{id}
fn delete_comment(
    repo: &mut CommentRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("comment" / String)
        .and(warp::delete())
        .and(with_repository(repo.clone()))
        .and_then(controllers::comment::delete_comment)
}

/// method: POST
/// path: /comment
/// body: { Comment }
fn create_comment(
    repo: &mut CommentRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("comment")
        .and(warp::post())
        .and(json_body::<Comment>())
        .and(with_repository(repo.clone()))
        .and_then(controllers::comment::add_comment)
}

/// method: GET
/// path: /comments
fn get_comments(
    repo: &mut CommentRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("comments")
        .and(warp::get())
        .and(with_repository(repo.clone()))
        .and_then(controllers::comment::get_comments)
}

/// method: GET
/// path: /comments/story/{id}
fn get_all_story_comments(
    repo: CommentRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("comments" / "story" / String)
        .and(warp::get())
        .and(warp::any().map(move || repo.clone()))
        .and_then(controllers::comment::get_all_story_comments)
}
