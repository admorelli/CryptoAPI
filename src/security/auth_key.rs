use crate::models::account::Account;
use crate::models::algorithm::{Alg, Algorithm};
use crate::models::diesel_db::Db;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

use diesel::prelude::*;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::gen::OpenApiGenerator;

#[derive(OpenApiFromRequest)]
pub struct ApiKey {
    pub user: Account,
    pub algorithms: Vec<Alg>,
}

#[derive(Debug)]
pub enum ApiKeyError {
    ConnectionFailed,
    AlgorithmMissing,
    Missing,
    Invalid,
}

impl<'r> OpenApiFromRequest<'r> for ApiKeyError {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKeyError {
    type Error = ApiKeyError;

    async fn from_request(_req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        Outcome::Success(Self::Missing)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        async fn find_user(key: &str, db: &Db) -> Result<Account, ApiKeyError> {
            use crate::models::account::account::dsl::*;
            let user_key = String::from(key);

            let users = db
                .run(move |conn| {
                    account
                        .filter(api_key.eq(user_key))
                        .filter(active.eq(true))
                        .load::<Account>(conn)
                })
                .await.or_else(|e| {
                println!("{}", e.to_string());
                Err(ApiKeyError::Invalid)
            })?;

            if users.len() > 0 {
                Ok(users.first().unwrap().clone())
            } else {
                Err(ApiKeyError::Invalid)
            }
        }

        async fn find_algs(key: i32, db: &Db) -> Result<Vec<Alg>, ApiKeyError> {
            use crate::models::algorithm::algorithm::dsl::*;
            use crate::models::algorithm::user_algorithm::dsl::*;

            let algorithms = db
                .run(move |conn| {
                    algorithm
                        .inner_join(user_algorithm)
                        .filter(user_id.eq(key))
                        .select((id, crypto, salting))
                        .load::<Algorithm>(conn)
                })
                .await.or(Err(ApiKeyError::Invalid))?;
            if algorithms.len() > 0 {
                Ok(algorithms
                    .into_iter()
                    .map(|r| Alg::try_from(r).unwrap())
                    .collect::<Vec<Alg>>())
            } else {
                Err(ApiKeyError::AlgorithmMissing)
            }
        }

        let key = req.headers()
                     .get_one("x-api-key")
            //.ok_or(Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)))?;
                     .unwrap();
        let db = match Db::from_request(req).await {
            Outcome::Success(conn) => conn,
            _ => {
                return Outcome::Failure((
                    Status::BadRequest,
                    ApiKeyError::ConnectionFailed,
                ))
            }
        };
        let user = match find_user(&key, &db).await {
            Ok(o) => o,
            Err(e) => return Outcome::Failure((Status::BadRequest, e))
        };

        let algorithms = match find_algs(user.id, &db).await {
            Ok(o) => o,
            Err(e) => return Outcome::Failure((Status::BadRequest, e))
        };
        Outcome::Success(ApiKey { user, algorithms })
    }
}
