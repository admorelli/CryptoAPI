use std::collections::BTreeMap;
use std::{collections::hash_map::DefaultHasher};
use std::hash::{Hash, Hasher};
use bb8::{Pool};
use bb8_surrealdb::SurrealdbConnectionManager;
use rocket::{response::status::*, State};

use crate::security::auth_key::ApiKey;
use crate::models::categoria::schema::model as categoria_base;
use crate::models::utils::SurrealdbQuery;

#[get("/")]
pub async fn index(_key: ApiKey, pool: &State<Pool<SurrealdbConnectionManager>>) -> Result<String, String>{
    let localpool = pool.get().await.unwrap();
    let results = categoria_base.select(localpool, None.into()).await;
    match results{
        Ok(r) => Ok(format!("{:#?}", r)),
        Err(e) => Err(e.to_string()),
        _ => Err("something".to_string())
    }
}

#[get("/<key>")]
pub async fn get(api_key: ApiKey, pool: &State<Pool<SurrealdbConnectionManager>>, key: &str) -> Result<Accepted<String>, NotFound<String>>{
    let mut hasher = DefaultHasher::new();
    format!("{}:{}", key, api_key.user.salt.to_string()).hash(&mut hasher);
    let hash = hasher.finish().to_string();
    let localpool = pool.get().await.unwrap();
    let res = categoria_base.select_by_key(localpool, hash.as_str()).await;
    //let res = select(localpool, model, Some(filter)).await;
    match res{
        Ok(r) => Ok(Accepted(Some("Record found".to_string()))),
        Err(r) => Err(NotFound(r.to_string()))
    }

}

#[post("/<key>")]
pub async fn add(_key: ApiKey, pool: &State<Pool<SurrealdbConnectionManager>>, key: &str) -> Result<Created<String>, Unauthorized<String>>{
    let mut hasher = DefaultHasher::new();
    format!("{}:{}", key, "teste".to_string()).hash(&mut hasher);
    // let hash = hasher.finish().to_string();
    // let localpool = pool.get().await.unwrap();
    // let model = Categoria{
    //     id: "teste",
    //     is_unsafe: true,
    //     secret: "segredo_secreto",
    //     owner: User{
    //         id: "teste",
    //         active: true,
    //         salt: "segredo_mais_secreto"
    //     }
    // };
    // let result = insert(localpool, model).await;
    // match result{
    //     Ok(r) => {
    //         if r.len() > 0{
    //             Ok(Created::new("Inserted with success".to_string()))
    //         }else{
    //             Err(Unauthorized(Some("Failed to insert".to_string())))
    //         }
    //     },
    //     Err(e) => Err(Unauthorized(Some(e.to_string())))
    // }
    Err(Unauthorized(Some("not implemented".to_string())))
}

#[delete("/<key>")]
pub fn del(_key: ApiKey, key: &str) -> String{
    format!("Deleted item with key '{}'",key)
}