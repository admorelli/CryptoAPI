use serde::{Deserialize, Serialize};
use crate::models::crypto::{ApplyCripto, plain::*, sha1::*};
use crate::models::salting::SaltingStrategy;
use crate::models::util::EnumStringParse;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CryptoAlgorithm {
    Plain(PlainTextAlgorithmn),
    Sha1(Sha1Algorithm)
}

impl CryptoAlgorithm {
    pub fn apply(&self, key: &str, salt: &Vec<&str>, salter: &SaltingStrategy) -> String {
        match self {
            CryptoAlgorithm::Plain(c) => c.apply(key, salt, &salter),
            CryptoAlgorithm::Sha1(c) => c.apply(key, salt, &salter)
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
            "sha1" => Some(CryptoAlgorithm::Sha1(Sha1Algorithm::default())),
            _ => Option::None,
        }
    }
}