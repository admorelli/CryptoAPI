use rocket::http::Status;
use rocket::log::private::debug;
use rocket::request::{Outcome, Request, FromRequest};
use crate::models::algorithm::{Algorithm, Alg};
use crate::models::diesel_sqlite::Db;
use crate::models::user::User;

use diesel::prelude::*;

pub struct ApiKey{
    pub user: User,
    pub algorithms: Vec<Alg>
}

#[derive(Debug)]
pub enum ApiKeyError {
    ConnectionFailed,
    AlgorithmMissing,
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        /// Returns true if `key` is a valid API key string.
        async fn find_user(key: &str, db: &Db) -> Result<User, ApiKeyError> {
            
            use crate::models::user::user::dsl::*;
            use diesel::QueryDsl;
            let user_key = String::from(key);

            let user_result = db.run(move |conn| {
                user
                .filter(api_key.eq(user_key))
                .filter(active.eq(true))
                .load::<User>(conn)
            }).await;
            
            match user_result {
                Ok(users) =>{
                    if users.len() > 0{
                        Ok(users.first().unwrap().clone())
                    }
                    else{
                        Err(ApiKeyError::Invalid)
                    }
                },
                Err(_e) => {
                    println!("{}", _e);
                    Err(ApiKeyError::Invalid)
                }
            }
        }
        async fn find_algs(key: i32, db: &Db) -> Result<Vec<Alg>, ApiKeyError>{
            use crate::models::algorithm::algorithm::dsl::*;
            use crate::models::algorithm::user_algorithm::dsl::*;
            
            let algorithms = db.run(move |conn| {
                algorithm
                .inner_join(user_algorithm)
                .filter(user_id.eq(key))
                .select((id, crypto, salting))
                .load::<Algorithm>(conn)
            }).await;

            match algorithms{
                Ok(results)=> {
                    if results.len() > 0{
                        Ok(results
                            .into_iter()
                            .map(|r| Alg::try_from(r).unwrap())
                            .collect::<Vec<Alg>>())
                    }else{
                        Err(ApiKeyError::AlgorithmMissing)
                    }
                },
                Err(_e) => Err(ApiKeyError::Invalid)
            }
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            Some(key) => {
                let db = match Db::from_request(req).await{
                    Outcome::Success(conn) => conn,
                    _ =>return Outcome::Failure((Status::BadRequest, ApiKeyError::ConnectionFailed))
                };
                let user = match find_user(&key, &db).await {
                    Ok(result) => result,
                    Err(e) => return Outcome::Failure((Status::BadRequest, e))
                };
                let algorithms = match find_algs(user.id, &db).await {
                    Ok(result) => result,
                    Err(e) => return Outcome::Failure((Status::BadRequest, e))
                };
                Outcome::Success(ApiKey{ user, algorithms})
            }
        }
    }
}