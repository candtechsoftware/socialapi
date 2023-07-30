use std::borrow::BorrowMut;

use warp::Filter;

use crate::{
    controllers,
    models::story::Story,
    respository::{story::StoryRepository, DbPool},
};

use super::{json_body, with_repository};

pub fn create_story_routes(
    dbpool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let story_repository = StoryRepository { 0: dbpool };
    get_story(story_repository.clone().borrow_mut())
        .or(create_story(story_repository.clone()))
        .or(update_story(story_repository.clone().borrow_mut()))
        .or(delete_story(story_repository.clone().borrow_mut()))
        .or(get_stories(story_repository.clone().borrow_mut()))
        .or(get_user_stories(story_repository.clone()))
}

/// method: GET
/// path: /story/{id}
fn get_story(
    repo: &mut StoryRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("story" / String)
        .and(warp::get())
        .and(with_repository(repo.clone()))
        .and_then(controllers::story::get_story)
}

/// method: PUT
/// path: /story/{id}
fn update_story(
    repo: &mut StoryRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("story" / String)
        .and(warp::put())
        .and(json_body::<Story>())
        .and(with_repository(repo.clone()))
        .and_then(controllers::story::update_story)
}

/// method: DELETE
/// path: /story/{id}
fn delete_story(
    repo: &mut StoryRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("story" / String)
        .and(warp::delete())
        .and(with_repository(repo.clone()))
        .and_then(controllers::story::delete_story)
}

/// method: POST
/// path: user/{id}/story      TODO: Find a better place for this route
/// body: { Story }
fn create_story(
    repo: StoryRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / String / "story")
        .and(warp::post())
        .and(json_body::<Story>())
        .and(warp::any().map(move || repo.clone()))
        .and_then(controllers::story::add_story)
}

/// method: GET
/// path: user/{id}/stories     TODO: Find a better place for this route
fn get_user_stories(
    repo: StoryRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("user" / String / "story")
        .and(warp::get())
        .and(warp::any().map(move || repo.clone()))
        .and_then(controllers::story::get_user_stories)
}

/// method: GET
/// path: /storys
fn get_stories(
    repo: &mut StoryRepository,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("stories")
        .and(warp::get())
        .and(with_repository(repo.clone()))
        .and_then(controllers::story::get_stories)
}
