use rocket::serde::{Deserialize, Serialize};

use crate::security::auth_key::ApiKey;

use super::diesel_sqlite::Db;
use super::util::FromDb;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "categoria"]
pub struct Categoria {
    pub id: String,
    pub is_unsafe: bool,
    pub salt: String,
    pub owner: i32,
}

table! {
    categoria (id) {
        id -> VarChar,
        is_unsafe -> Bool,
        salt -> VarChar,
        owner -> Integer,
    }
}

#[derive(Debug)]
pub enum CategoriaError {
    ConnectionFailed,
    NotFound,
}

#[async_trait]
impl FromDb for Categoria {
    type Error = CategoriaError;
    async fn from_db(
        key: &String,
        db: &Db,
        api_key: &ApiKey,
    ) -> Result<Vec<Categoria>, Self::Error> {
        let user_id = api_key.user.id.clone();
        let hashes = api_key
            .algorithms
            .clone()
            .into_iter()
            .map(|a| a.apply(key.as_str(), &vec![api_key.user.salt.as_str()]))
            .collect::<Vec<_>>();
        let results = db
            .run(move |conn| {
                use crate::models::categoria::categoria::dsl::*;
                use diesel::prelude::*;

                categoria
                    .filter(id.eq_any(hashes))
                    .filter(owner.eq(user_id))
                    .load::<Categoria>(conn)
            })
            .await;

        match results {
            Ok(categorias) => {
                if categorias.len() > 0 {
                    Ok(categorias)
                } else {
                    Err(Self::Error::NotFound)
                }
            }
            Err(e) => {
                println!("{}", e.to_string());
                Err(Self::Error::ConnectionFailed)
            }
        }
    }
}
