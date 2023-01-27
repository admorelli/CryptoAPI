use serde::{Deserialize, Serialize};
use crate::models::crypto::ApplyCripto;
use crate::models::salting::SaltingStrategy;
use sha1::{Sha1, Digest};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Default)]
pub struct Sha1Algorithm;

impl ApplyCripto for Sha1Algorithm {
    fn apply(&self, key: &str, salt: &Vec<&str>, salter: &SaltingStrategy) -> String {
        let mut hasher = Sha1::new();
        hasher.update(salter.apply(key, &salt).as_str());
        let result = hasher.finalize();
        format!("{:X?}", result)
    }
}