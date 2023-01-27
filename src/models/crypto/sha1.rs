use serde::{Deserialize, Serialize};
use crate::models::crypto::ApplyCripto;
use crate::models::salting::SaltingStrategy;
use rocket::tokio::

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Default)]
pub struct Sha1Algorithm;

impl ApplyCripto for Sha1Algorithm {
    fn apply(&self, key: &str, salt: &Vec<&str>, salter: &SaltingStrategy) -> String {
        salter.apply(key, &salt)
    }
}
