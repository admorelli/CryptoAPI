use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
use crate::models::user::User;

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
        async fn find_user<'a>(key: &'a str) -> Outcome<ApiKey, ApiKeyError> {
            if key == "valid_api_key" {
                Outcome::Success(ApiKey{ user: User::default()})
            }else{
                Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
            }
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) => {
                find_user(key).await
            }
        }
    }
}