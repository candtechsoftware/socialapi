use diesel;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::story;

#[derive(
    Debug,
    Clone,
    AsChangeset,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Insertable,
    Selectable,
)]
#[diesel(primary_key(id))]
#[diesel(table_name = story)]
pub struct Story {
    #[diesel(deserialize_as = i32)]
    id: Option<i32>,
    data: String,
}
