pub mod comment;
pub mod story;
pub mod user;

use std::convert::Infallible;

use serde::de::DeserializeOwned;
use warp::Filter;

use crate::respository::{DbPool, RespositoryTrait};

use comment::create_comment_routes;
use story::create_story_routes;
use user::create_user_routes;

pub fn json_body<T: DeserializeOwned + Send>(
) -> impl warp::Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn with_repository<T>(
    respository: impl RespositoryTrait<T>,
) -> impl warp::Filter<Extract = (impl RespositoryTrait<T>,), Error = Infallible> + Clone {
    warp::any().map(move || respository.clone())
}

pub fn create_routes(
    dbpool: DbPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let log = warp::log::custom(|info| {
        eprintln!("{} {} {:?}", info.method(), info.path(), info.elapsed())
    });
    warp::path::end()
        .map(index)
        .with(log)
        .or(create_user_routes(dbpool.clone()))
        .or(create_story_routes(dbpool.clone()))
        .or(create_comment_routes(dbpool.clone()))
}

pub fn index() -> &'static str {
    "Hello world!"
}
