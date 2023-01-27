use rocket::serde::{Deserialize, Serialize};

use super::salting::SaltingStrategy;
use super::util::EnumStringParse;
use super::crypto::enumerator::CryptoAlgorithm;

#[derive(Debug, Clone)]
pub struct Alg {
    pub id: i32,
    pub crypto: CryptoAlgorithm,
    pub salting: SaltingStrategy,
}

impl Alg {
    pub fn apply(&self, key: &str, salt: &Vec<&str>) -> String {
        self.crypto.apply(key, salt, &self.salting)
    }
}

impl TryFrom<Algorithm> for Alg {
    type Error = super::util::Error;

    fn try_from(value: Algorithm) -> Result<Self, Self::Error> {
        let crypto = CryptoAlgorithm::string_to_enum(value.crypto).ok_or(super::util::Error(Some("Failed to parse CryptoAlgorithm".to_string())))?;
        let salting = SaltingStrategy::string_to_enum(value.salting).ok_or(super::util::Error(Some("Failed to Parse SaltingStrategy".to_string())))?;
        Ok(Self {
            id: value.id,
            crypto,
            salting,
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "algorithm"]
pub struct Algorithm {
    pub id: i32,
    //api key to be sent
    pub crypto: String,
    //secret used for authentication
    pub salting: String,
}

impl From<Alg> for Algorithm {
    fn from(value: Alg) -> Self {
        Self {
            id: value.id,
            crypto: value.crypto.enum_to_string(),
            salting: value.salting.enum_to_string(),
        }
    }
}

table! {
        algorithm (id) {
            id -> Integer,
            crypto -> VarChar,
            salting -> VarChar,
        }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "user_algorithm"]
pub struct UserAlgorithm {
    pub user_id: i32,
    pub algorithm_id: i32,
    pub ordering: i32,
}

table! {
    user_algorithm(user_id, algorithm_id){
        user_id -> Integer,
        algorithm_id -> Integer,
        ordering -> Integer,
    }
}

joinable!(user_algorithm -> algorithm(algorithm_id));
allow_tables_to_appear_in_same_query!(user_algorithm, algorithm);
