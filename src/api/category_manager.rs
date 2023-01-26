use diesel::{delete, prelude::*};
use rocket::response::status::*;
use rocket::serde::json::Json;

use crate::models::categoria::{Categoria, CategoriaError};
use crate::models::diesel_db::Db;
use crate::security::auth_key::ApiKey;

#[get("/")]
pub async fn index(key: ApiKey, db: Db) -> Result<Json<Vec<Categoria>>, String> {
    use crate::models::categoria::categoria::dsl::*;
    use diesel::QueryDsl;

    let result = db
        .run(move |conn| {
            categoria
                .filter(owner.eq(key.user.id))
                .load::<Categoria>(conn)
        })
        .await;

    match result {
        Ok(categorias) => Ok(Json(categorias)),
        Err(e) => Err(e.to_string()),
    }
    //correct message
    //Ok("Endpoint to manage categorias")
}

#[get("/<key>")]
pub async fn get(
    api_key: ApiKey,
    db: Db,
    key: String,
) -> Result<Accepted<String>, NotFound<String>> {
    let result = Categoria::from_db(&key, &db, &api_key, None).await;
    match result {
        Ok(res) => {
            if res.len() > 0 {
                Ok(Accepted(Some("Record found.".into())))
            } else {
                Err(NotFound("Record not found.".into()))
            }
        }
        Err(_) => Err(NotFound("Failed to communicate to the database".into())),
    }
}

#[post("/<key>")]
pub async fn add(
    api_key: ApiKey,
    db: Db,
    key: String,
) -> Result<Created<String>, Unauthorized<String>> {
    use crate::models::categoria::categoria::dsl::*;
    use rand::distributions::{Alphanumeric, DistString};

    let categorias = Categoria::from_db(&key, &db, &api_key, None).await;
    match categorias {
        Ok(_) => return Err(Unauthorized(Some("Category already exists.".into()))),
        Err(e) => match e {
            CategoriaError::ConnectionFailed => {
                return Err(Unauthorized(Some(
                    "Failed to connect to the database.".into(),
                )))
            }
            _ => (),
        },
    };
    let result = db
        .run(move |conn| {
            let alg = api_key.algorithms.first().unwrap();
            let hash = alg.apply(key.as_str(), &vec![api_key.user.salt.as_str()]);
            let generated_salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

            diesel::insert_into(categoria)
                .values((
                    id.eq(hash.as_str()),
                    is_unsafe.eq(false),
                    salt.eq(generated_salt.as_str()),
                    owner.eq(api_key.user.id),
                ))
                .execute(conn)
        })
        .await;

    match result {
        Ok(_) => Ok(Created::new("Success")),
        Err(e) => {
            println!("{:?}", e);
            Err(Unauthorized(Some("Failed to insert".into())))
        }
    }
}

#[delete("/<key>")]
pub async fn del(
    api_key: ApiKey,
    db: Db,
    key: String,
) -> Result<Accepted<String>, Unauthorized<String>> {
    use crate::models::categoria::categoria::dsl::*;

    let categorias = Categoria::from_db(&key, &db, &api_key, None).await;

    match categorias {
        Ok(cats) => {
            let keys = cats.clone().into_iter().map(|c| c.id).collect::<Vec<_>>();
            let res = db
                .run(move |conn| {
                    use crate::models::hash::hash::dsl::*;
                    use diesel::dsl::{exists, select};
                    select(exists(hash.filter(owner.eq_any(keys))))
                        .get_result::<bool>(conn)
                })
                .await.expect("Failed to communicate to the database");

            if res {
                return Err(Unauthorized(Some(
                    "The category still has items, aborting.".into(),
                )));
            }

            let keys2 = cats.into_iter().map(|c| c.id).collect::<Vec<_>>();
            let _success = db
                .run(|conn| delete(categoria).filter(id.eq_any(keys2)).execute(conn))
                .await.expect("Failed to communicate to the database");
            Ok(Accepted(Some("Record found.".into())))
        }
        Err(e) => match e {
            CategoriaError::NotFound => Err(Unauthorized(Some("Record not found.".into()))),
            CategoriaError::ConnectionFailed => {
                Err(Unauthorized(Some(
                    "Failed to communicate to the database".into(),
                )))
            }
        }
    }
}
