use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="categoria"]
pub struct Categoria{
    pub id: String,
    pub is_unsafe: bool,
    pub salt: String,
    pub owner: i32
}

table! {
    categoria (id) {
        id -> VarChar,
        is_unsafe -> Bool,
        salt -> VarChar,
        owner -> Integer,
    }
}