use rocket::serde::{Serialize, Deserialize};

use super::crypto::CryptoAlgorithm;
use super::salting::SaltingStrategy;
use super::util::EnumStringParse;


#[derive(Debug, Clone)]
pub struct Alg{
    pub id: String,
    pub crypto: CryptoAlgorithm,
    pub salting: SaltingStrategy
}

impl  TryFrom<Algorithm> for Alg {
    
    type Error = super::util::Error;

    fn try_from(value: Algorithm) -> Result<Self, Self::Error> {
        match CryptoAlgorithm::string_to_enum(value.crypto){
            Some(crypto) => {
                match SaltingStrategy::string_to_enum(value.salting){
                    Some(salting) => Ok(Self { id: value.id, crypto, salting }),
                    None => Err(super::util::Error(Some("Conversion Failed".to_string())))
                }
            },
            None => Err(super::util::Error(Some("Conversion Failed".to_string())))
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="algorithm"]
pub struct Algorithm{
        pub id: String,
        //api key to be sent
        pub crypto: String,
        //secret used for authentication
        pub salting: String
}



impl  From<Alg> for Algorithm {
    fn from(value: Alg) -> Self {
        Self { id: value.id, crypto:value.crypto.enum_to_string(), salting: value.salting.enum_to_string() }
    }
}

table! {
        algorithm (id) {
            id -> VarChar,
            crypto -> VarChar,
            salting -> VarChar,
        }
}