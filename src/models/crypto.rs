use serde::{Deserialize, Serialize};

use super::{salting::SaltingStrategy, util::EnumStringParse};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Default)]
pub struct PlainTextAlgorithmn;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CryptoAlgorithm {
    Plain(PlainTextAlgorithmn),
}

impl CryptoAlgorithm {
    pub fn apply(&self, key: &str, salt: &Vec<&str>, salter: &SaltingStrategy) -> String {
        match self {
            CryptoAlgorithm::Plain(c) => c.apply(key, salt, &salter),
        }
    }
}

impl Default for CryptoAlgorithm {
    fn default() -> Self {
        CryptoAlgorithm::Plain(PlainTextAlgorithmn::default())
    }
}

impl EnumStringParse for CryptoAlgorithm {
    fn str_to_enum(key: &str) -> Option<Self> {
        match key {
            "plain" => Some(CryptoAlgorithm::Plain(PlainTextAlgorithmn::default())),
            _ => Option::None,
        }
    }
}

trait ApplyCripto {
    fn apply(&self, key: &str, salt: &Vec<&str>, salter: &SaltingStrategy) -> String;
}

impl ApplyCripto for PlainTextAlgorithmn {
    fn apply(&self, key: &str, salt: &Vec<&str>, salter: &SaltingStrategy) -> String {
        salter.apply(key, &salt)
    }
}
