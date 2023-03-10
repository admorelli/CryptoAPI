use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "account"]
pub struct Account {
    #[serde(skip_deserializing)]
    pub id: i32,
    //api key to be sent
    pub api_key: String,
    //salt to be used by the user child objects(Categories, Hashes and Storages)
    pub salt: String,
    //set if the user is active
    pub active: bool,
}

table! {
        account (id) {
            id -> Integer,
            api_key -> VarChar,
            salt -> VarChar,
            active -> Bool,
        }
}
