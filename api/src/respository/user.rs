use diesel;
use diesel::prelude::*;

use crate::models::user::UserAccount;

use super::{DbPool, RespositoryResult, RespositoryTrait};

#[derive(Clone)]
pub struct UserAccountRepository(pub(crate) DbPool);

impl RespositoryTrait<UserAccount> for UserAccountRepository {
    fn create(&mut self, user: UserAccount) -> Result<UserAccount, diesel::result::Error> {
        use crate::schema::user_account;
        diesel::insert_into(user_account::table)
            .values(&user)
            .get_result::<UserAccount>(&mut self.0.get().expect("Error getting connection"))
    }

    fn get_all(&mut self) -> RespositoryResult<Vec<UserAccount>> {
        use crate::schema::user_account::dsl::*;
        let users = user_account
            .load::<UserAccount>(&mut self.0.get().expect("Error getting connection"))
            .expect("Error listing all users");
        Ok(users)
    }

    fn get_one(&mut self, model_id: i32) -> RespositoryResult<UserAccount> {
        use crate::schema::user_account::dsl::*;
         match user_account.find(model_id)
            .first::<UserAccount>(&mut self.0.get().expect("Error getting connection")) {
                Ok(user) => Ok(user),
                Err(err) => Err(err)
            }
    }

    fn update(&mut self, model_id: i32, model: UserAccount) -> RespositoryResult<UserAccount> {
        use crate::schema::user_account::dsl::*;
        diesel::update(user_account.find(model_id))
            .set(&model)
            .get_result::<UserAccount>(&mut self.0.get().expect("Error getting connection"))
    }

    fn delete(&mut self, model_id: i32) -> super::RespositoryResult<()> {
        use crate::schema::user_account::dsl::*;
        diesel::delete(user_account.find(model_id))
            .execute(&mut self.0.get().expect("Error getting connection"))
            .expect("Error deleting a single item");
        Ok(())
    }
}
