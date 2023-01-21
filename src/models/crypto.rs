use serde::{Deserialize, Serialize};

use super::util::EnumStringParse;


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Default)]
pub struct PlainTextAlgorithmn;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CryptoAlgorithm{
    Plain(PlainTextAlgorithmn)
}

impl Default for CryptoAlgorithm{
    fn default() -> Self{
        CryptoAlgorithm::Plain(PlainTextAlgorithmn::default())
    }
}

impl EnumStringParse for CryptoAlgorithm{
    fn str_to_enum(key: &str) -> Option<Self>{
        match key {
            "plain" => Some(CryptoAlgorithm::Plain(PlainTextAlgorithmn::default())),
            _ => Option::None
        }
    }
}