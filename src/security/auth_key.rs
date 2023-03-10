use crate::models::account::Account;
use crate::models::algorithm::{Alg, Algorithm};
use rocket::http::{Status};
use rocket::request::{FromRequest, Outcome, Request};

use diesel::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::gen::OpenApiGenerator;
use crate::models;
use crate::models::diesel_db::Db;

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
                error!("account search: {}", e.to_string());
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
                        .order(ordering.asc())
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

        async fn get_authentication(req: &Request<'_>) -> Result<String, ApiKeyError> {
            get_user_rapid_api(req).await.or(get_user_rapid_api(req).await)
        }

        async fn get_user_api_key(req: &Request<'_>) -> Result<String, ApiKeyError> {
            let key = req.headers().get_one("x-api-key").ok_or(ApiKeyError::Missing)?;
            Ok(key.to_string())
        }

        async fn get_user_rapid_api(req: &Request<'_>) -> Result<String, ApiKeyError> {
            const PROXY_SECRET: &str = "782282f0-aadf-11ed-9c04-3b47f63222ea";
            let rapid_token = req.headers().get_one("X-RapidAPI-Key").or(Some("rapid_api")).unwrap();
            let rapid_secret = req.headers().get_one("X-RapidAPI-Proxy-Secret").ok_or(ApiKeyError::Missing)?;
            let rapid_user = req.headers().get_one("X-RapidAPI-User").ok_or(ApiKeyError::Missing)?;

            if rapid_secret != PROXY_SECRET {
                return Err(ApiKeyError::Invalid);
            }

            let db = match Db::from_request(req).await {
                Outcome::Success(conn) => conn,
                _ => return Err(ApiKeyError::ConnectionFailed),
            };

            //todo: validate token

            let username = format!("{}:{}", rapid_token, rapid_user);
            let query_username = username.clone();

            db.run(move |conn| {
                use models::account::account::dsl::*;
                use diesel::dsl::*;
                let account_exists: bool = match select(exists(account.filter(api_key.eq(&query_username))))
                    .get_result(conn) {
                    Ok(r) => r,
                    Err(e) => {
                        error!("{:?}", e);
                        false
                    },
                };

                let generated_salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

                if account_exists == false {
                    let inserted_id: i32 = diesel::insert_into(account)
                        .values((
                            api_key.eq(query_username),
                            salt.eq(generated_salt.as_str()),
                            active.eq(true),
                        ))
                        .returning(id)
                        .get_result(conn)?;
                    //inserts a default algorithm

                    use models::algorithm::user_algorithm::dsl::*;
                    let success = diesel::insert_into(user_algorithm)
                        .values((
                            user_id.eq(inserted_id),
                            algorithm_id.eq(4),//todo: make it not be hard coded
                            ordering.eq(0),
                        )).execute(conn)?;

                    if success == 0 {
                        Ok(inserted_id)
                    } else {
                        Err(diesel::result::Error::NotFound)
                    }
                } else {
                    Ok(0)
                }
            }).await.or_else(|e| {
                error!("failed to insert account:{:?}", e);
                Err(ApiKeyError::ConnectionFailed)
            })?;

            Ok(username)
        }

        let db = match Db::from_request(req).await {
            Outcome::Success(conn) => conn,
            _ => {
                return Outcome::Failure((
                    Status::Unauthorized,
                    ApiKeyError::ConnectionFailed,
                ))
            }
        };

        let username = match get_authentication(&req).await {
            Ok(user) => user,
            Err(error) => return Outcome::Failure((Status::Unauthorized, error)),
        };
        let user = match find_user(&username, &db).await {
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
