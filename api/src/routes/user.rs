use std::borrow::BorrowMut;

use warp::Filter;

use crate::{
    controllers,
    models::user::UserAccount,
    respository::{user::UserAccountRepository, DbPool},
};

use super::{json_body, with_repository};

pub fn create_user_routes(
    dbpool: DbPool,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let user_repository = UserAccountRepository { 0: dbpool.clone() };
    get_user(user_repository.clone().borrow_mut())
        .or(update_user(user_repository.clone().borrow_mut()))
        .or(delete_user(user_repository.clone().borrow_mut()))
        .or(create_user(user_repository.clone().borrow_mut()))
        .or(get_users(user_repository.clone().borrow_mut()))
}

/// method: GET
/// path: /user/{id}
fn get_user(
    repo: &mut UserAccountRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("user" / String)
        .and(warp::get())
        .and(with_repository(repo.clone()))
        .and_then(controllers::user::get_user)
}

/// method: PUT
/// path: /user/{id}
fn update_user(
    repo: &mut UserAccountRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("user" / String)
        .and(warp::put())
        .and(json_body::<UserAccount>())
        .and(with_repository(repo.clone()))
        .and_then(controllers::user::update_user)
}

/// method: DELETE
/// path: /user/{id}
fn delete_user(
    repo: &mut UserAccountRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("user" / String)
        .and(warp::delete())
        .and(with_repository(repo.clone()))
        .and_then(controllers::user::delete_user)
}

/// method: POST
/// path: /user
/// body: { UserAccount }
fn create_user(
    repo: &mut UserAccountRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("user")
        .and(warp::post())
        .and(json_body::<UserAccount>())
        .and(with_repository(repo.clone()))
        .and_then(controllers::user::add_user)
}

/// method: GET
/// path: /users
fn get_users(
    repo: &mut UserAccountRepository,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("users")
        .and(warp::get())
        .and(with_repository(repo.clone()))
        .and_then(controllers::user::get_users)
}
