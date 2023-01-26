use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::algorithm::Alg;
use super::diesel_db::Db;
use crate::models::categoria::{Categoria};
use crate::security::auth_key::ApiKey;

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

#[derive(Debug)]
pub enum HashError {
    ConnectionFailed,
    NotFound,
}

impl std::fmt::Display for HashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Hash {
    pub async fn from_db(
        key: (&String, &String),
        category: &String,
        db: &Db,
        api_key: &ApiKey,
        only_valid: Option<bool>,
    ) -> Result<(Alg, Categoria, Hash), HashError> {
        use crate::models::hash::hash::dsl::*;
        use diesel::prelude::*;

        let categorias = Categoria::from_db(&category, &db, &api_key, only_valid.clone()).await.expect("Category not found");
        for categoria in categorias {
            let hash_dic = api_key
                .algorithms
                .clone()
                .into_iter()
                .map(|a| {
                    (
                        a.apply(
                            key.0.as_str(),
                            &vec![api_key.user.salt.as_str(), categoria.salt.as_str()],
                        ),
                        a.clone(),
                    )
                })
                .collect::<HashMap<_, _>>();
            let category_id = categoria.id.clone();
            let hashes = hash_dic.clone().into_keys().collect::<Vec<_>>();
            let valid = !only_valid.unwrap_or(false);
            let hash_model = db
                .run(move |conn| {
                    hash.filter(id.eq_any(hashes))
                        .filter(owner.eq(category_id))
                        .filter(is_unsafe.eq_any(vec![valid, false]))
                        .load::<Hash>(conn)
                })
                .await.expect("Connection Failed");

            if hash_model.len() > 0 {
                let hh = hash_model.first().unwrap();
                let alg = &hash_dic[&hh.id];
                let hashed_data_string = alg.apply(key.1, &vec![&api_key.user.salt, &categoria.salt, &hh.salt]);
                if hashed_data_string == hh.hashed_data {
                    return Ok((alg.clone(), categoria, hh.clone()));
                } else {
                    return Err(HashError::NotFound)
                }
            }
        }
        Err(HashError::NotFound)
    }
}
