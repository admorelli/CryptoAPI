use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "hash"]
pub struct Hash {
    //calculated by the key of a model(not saved) + user salt + category salt +its own salt
    //calculated by one of the algmodels associated to the category
    pub id: String,
    //define if it is safe to use
    pub is_unsafe: bool,
    //salt used fo calculate its own key and its children(Hash and Storage)
    pub salt: String,
    //hashed data to compare against
    pub hashed_data: String,
    //category that own the hash
    pub owner: String,
}

table! {
    hash (id) {
        id -> VarChar,
        is_unsafe -> Bool,
        salt -> VarChar,
        hashed_data -> VarChar,
        owner -> VarChar,
    }
}
