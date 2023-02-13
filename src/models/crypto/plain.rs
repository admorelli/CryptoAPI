use serde::{Deserialize, Serialize};
use crate::models::crypto::ApplyCripto;
use crate::models::salting::SaltingStrategy;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Default)]
pub struct PlainTextAlgorithmn;

impl ApplyCripto for PlainTextAlgorithmn {
    fn apply(&self, key: &str, salt: &Vec<&str>, salter: &SaltingStrategy) -> String {
        salter.apply(key, &salt)
    }
}