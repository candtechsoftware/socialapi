use diesel;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::user_account;

#[derive(AsChangeset, Serialize, Deserialize, Identifiable, Queryable, Insertable, Selectable)]
#[diesel(primary_key(id))]
#[diesel(table_name = user_account)]
pub struct UserAccount {
    #[diesel(deserialize_as = i32)]
    id: Option<i32>,
    username: String,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
}
