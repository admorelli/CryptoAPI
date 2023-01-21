use diesel::QueryDsl;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use crate::models::diesel_sqlite::Db;
use crate::models::user::User;

use diesel::prelude::*;

pub struct ApiKey{
    pub user: User
}

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        async fn find_user(key: String, db: Db) -> Outcome<ApiKey, ApiKeyError> {
            
            use crate::models::user::user::dsl::*;
            let result = db.run(move |conn| {
                user
                .filter(api_key.eq(key))
                .load::<User>(conn)
            }).await;
            
            match result {
                Ok(users) =>{
                    if users.len() > 0{
                        Outcome::Success(ApiKey{user:users.first().unwrap().clone()})
                    }
                    else{
                        Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
                    }
                },
                _ => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
            }
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) => find_user(key.to_string(), Db::from_request(req).await.unwrap()).await    
        }
    }
}