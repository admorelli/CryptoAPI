use rocket::serde::{Serialize, Deserialize};

use rocket_sync_db_pools::{diesel};

use self::diesel::prelude::*;


#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="user"]
pub struct User{
        #[serde(skip_deserializing)]
        pub id: i32,
        //api key to be sent
        pub api_key: String,
        //secret used for authentication
        pub secret: String,
        //salt to be used by the user child objects(Categories, Hashes and Storages)
        pub salt: String,
        //set if the user is active
        pub active: bool
}

table! {
        user (id) {
            id -> Integer,
            api_key -> VarChar,
            secret -> VarChar,
            salt -> VarChar,
            active -> Bool,
        }
    }

