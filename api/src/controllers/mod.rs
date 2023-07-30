pub mod comment;
pub mod story;
pub mod user;

use serde::{Deserialize, Serialize};
use std::convert::Infallible;

pub type ControllerResult<R> = Result<R, Infallible>;

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
    status: ResponseStatus,
    body: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub enum ResponseStatus {
    SUCCESS,
    FAILED,
}
