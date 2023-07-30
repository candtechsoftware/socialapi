pub mod comment;
pub mod story;
pub mod user;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub type RespositoryResult<T> = Result<T, diesel::result::Error>;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub trait RespositoryTrait<T>
where
    Self: Sized + Clone + Send,
{
    fn create(&mut self, model: T) -> RespositoryResult<T>;
    fn get_all(&mut self) -> RespositoryResult<Vec<T>>;
    fn get_one(&mut self, model_id: i32) -> RespositoryResult<T>;
    fn update(&mut self, model_id: i32, model: T) -> RespositoryResult<T>;
    fn delete(&mut self, model_id: i32) -> RespositoryResult<()>;
}
