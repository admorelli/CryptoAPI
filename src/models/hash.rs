use std::collections::HashMap;
use rocket::serde::{Deserialize, Serialize};

use super::algorithm::Alg;
use super::diesel_db::Db;
use crate::models::categoria::{Categoria, CategoriaError};
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
        key: &String,
        category: &String,
        db: &Db,
        api_key: &ApiKey,
    ) -> Result<(Alg, Categoria, Hash), HashError> {
        use crate::models::hash::hash::dsl::*;
        use diesel::prelude::*;

        let cats = Categoria::from_db(&category, &db, &api_key).await;

        match cats {
            Ok(categorias) => {
                for categoria in categorias {
                    let hash_dic = api_key
                        .algorithms
                        .clone()
                        .into_iter()
                        .map(|a| {
                            (a.apply(
                                key.as_str(),
                                &vec![api_key.user.salt.as_str(), categoria.salt.as_str()],
                            ), a.clone())
                        })
                        .collect::<HashMap<_, _>>();
                    let category_id = categoria.id.clone();
                    let hashes = hash_dic.clone().into_keys().collect::<Vec<_>>();
                    let result = db.run(move |conn|{
                        hash
                            .filter(id.eq_any(hashes))
                            .filter(owner.eq(category_id))
                            .load::<Hash>(conn)
                    }).await;

                    match result{
                        Ok(hash_model) =>{
                            if hash_model.len() > 0{
                                let hh = hash_model.first().unwrap();
                                let alg = &hash_dic[&hh.id];
                                return Ok((alg.clone(), categoria, hh.clone()))
                            }
                        },
                        Err(_e) => return Err(HashError::ConnectionFailed),
                    }
                }
                Err(HashError::NotFound)
            },
            Err(e) => match e {
                CategoriaError::NotFound => return Err(HashError::NotFound),
                CategoriaError::ConnectionFailed => return Err(HashError::ConnectionFailed),
            },
        }
    }
}
