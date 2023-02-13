use serde::{Deserialize, Serialize};
use crate::models::crypto::ApplyCripto;
use crate::models::salting::SaltingStrategy;
use sha2::{Sha256, Sha512, Digest};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Default)]
pub struct Sha256Algorithm;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Default)]
pub struct Sha512Algorithm;

impl ApplyCripto for Sha256Algorithm {
    fn apply(&self, key: &str, salt: &Vec<&str>, salter: &SaltingStrategy) -> String {
        let mut hasher = Sha256::new();
        hasher.update(salter.apply(key, &salt).as_str());
        let result = hasher.finalize();
        hex::encode(result)
    }
}

impl ApplyCripto for Sha512Algorithm {
    fn apply(&self, key: &str, salt: &Vec<&str>, salter: &SaltingStrategy) -> String {
        let mut hasher = Sha512::new();
        hasher.update(salter.apply(key, &salt).as_str());
        let result = hasher.finalize();
        format!("{:X?}", result)
    }
}