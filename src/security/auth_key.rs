use bb8::{Pool};
use bb8_surrealdb::SurrealdbConnectionManager;
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use crate::models::user::User;
use crate::models::utils::SurrealdbQuery;

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
        async fn find_user<'a>(key: &'a str, pool: &'a Pool<SurrealdbConnectionManager>) -> Outcome<ApiKey, ApiKeyError> {
            if key == "valid_api_key" {
                match pool.get().await {
                    Ok(conn) => {
                        match User::select_by_key(conn, key).await {
                            Ok(u) => {
                                Outcome::Success(ApiKey{ user: u})
                            },
                            Err(e) => {
                                //find a way to store error
                                Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
                            }
                        }
                        
                    }
                    Err(e) => {
                        //find a way to store error
                        Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
                    }
                }
            }else{
                Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
            }
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) => {
                find_user(key, req.rocket().state().unwrap()).await
            }
        }
    }
}