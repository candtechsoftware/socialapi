use warp;

use crate::models::user::UserAccount;
use crate::respository::RespositoryTrait;

use super::{ControllerResult, Response};

pub async fn add_user(
    new_user: UserAccount,
    repository: impl RespositoryTrait<UserAccount>,
) -> ControllerResult<impl warp::Reply> {
    match repository.clone().create(new_user) {
        Ok(user) => Ok(Box::new(warp::reply::json(&Response::<UserAccount> {
            status: super::ResponseStatus::SUCCESS,
            body: Some(user),
        }))),
        Err(err) => Ok(Box::new(warp::reply::json(&format!(
            "Error creating a user: {:?}",
            err
        )))),
    }
}

pub async fn get_user(
    user_id: String,
    repository: impl RespositoryTrait<UserAccount>,
) -> ControllerResult<impl warp::Reply> {
    let user = repository
        .clone()
        .get_one(user_id.parse().unwrap())
        .expect("Error getting single user");
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        body: Some(user),
    })))
}

pub async fn get_users(
    repository: impl RespositoryTrait<UserAccount>,
) -> ControllerResult<impl warp::Reply> {
    let users = repository
        .clone()
        .get_all()
        .expect("Error getting all users");
    Ok(Box::new(warp::reply::json(&Response {
        status: super::ResponseStatus::SUCCESS,
        body: Some(users),
    })))
}

pub async fn delete_user(
    user_id: String,
    repository: impl RespositoryTrait<UserAccount>,
) -> ControllerResult<impl warp::Reply> {
    repository
        .clone()
        .delete(user_id.parse().unwrap())
        .expect("Error deleting a user");
    Ok(Box::new(warp::reply::json(&Response::<UserAccount> {
        status: super::ResponseStatus::SUCCESS,
        body: None,
    })))
}

pub async fn update_user(
    user_id: String,
    updated_user_info: UserAccount,
    repository: impl RespositoryTrait<UserAccount>,
) -> ControllerResult<impl warp::Reply> {
    let user = repository
        .clone()
        .update(user_id.parse().unwrap(), updated_user_info)
        .expect("Error updatting a user");
    Ok(Box::new(warp::reply::json(&Response::<UserAccount> {
        status: super::ResponseStatus::SUCCESS,
        body: Some(user),
    })))
}
