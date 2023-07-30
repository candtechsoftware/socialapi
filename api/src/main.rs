mod controllers;
mod models;
mod respository;
mod routes;
mod schema;
mod services;

use respository::DbPool;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use warp::{http::StatusCode, Filter};

use crate::routes::create_routes;

fn create_db_pool(db_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::new(manager).expect("Postgress connection pool could not be started")
}

#[tokio::main]
async fn main() {
    pretty_env_logger::env_logger::init();
    let health_route = warp::path!("health").map(|| StatusCode::OK);

    let database_url = String::from("postgres://postgres:mysecretpassword@localhost/social_api");

    let pool = create_db_pool(database_url.as_str());
    let routes = create_routes(pool);
    info!("Starting server on port 8000");
    warp::serve(routes.or(health_route))
        .run(([127, 0, 0, 1], 8000))
        .await;
}
