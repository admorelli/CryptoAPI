use diesel::prelude::*;
use rocket::response::status::*;
use rocket::serde::json::Json;

use crate::models::categoria::Categoria;
use crate::models::diesel_sqlite::Db;
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
    use crate::models::categoria::categoria::dsl::*;

    let alg = api_key.algorithms.first().unwrap();
    let category_id = alg.crypto.apply(
        key.as_str(),
        &vec![api_key.user.salt.as_str()],
        &alg.salting,
    );
    println!("{}", category_id.as_str());
    let result = db
        .run(move |conn| {
            use diesel::dsl::{exists, select};
            select(exists(
                categoria
                    .filter(owner.eq(api_key.user.id))
                    .filter(id.eq(category_id.as_str())),
            ))
            .get_result::<bool>(conn)
        })
        .await;
    match result {
        Ok(res) => {
            if res {
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
    let result = db
        .run(move |conn| {
            let alg = api_key.algorithms.first().unwrap();
            let hash = alg.crypto.apply(
                key.as_str(),
                &vec![api_key.user.salt.as_str()],
                &alg.salting,
            );
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
        Err(_) => Err(Unauthorized(Some("Failed to insert".into()))),
    }
}

#[delete("/<key>")]
pub async fn del(
    api_key: ApiKey,
    db: Db,
    key: String,
) -> Result<Accepted<String>, Unauthorized<String>> {
    use crate::models::categoria::categoria::dsl::*;
    use diesel::dsl::{exists, select};
    let alg = api_key.algorithms.first().unwrap();
    let hash = alg.crypto.apply(
        key.as_str(),
        &vec![api_key.user.salt.as_str()],
        &alg.salting,
    );
    let result = db
        .run(move |conn| {
            select(exists(
                categoria
                    .filter(owner.eq(api_key.user.id))
                    .filter(id.eq(hash.as_str())),
            ))
            .get_result::<bool>(conn)
        })
        .await;

    match result {
        Ok(res) => {
            if res == false {
                return Err(Unauthorized(Some("Record not found.".into())));
            }
            let hash = alg.crypto.apply(
                key.as_str(),
                &vec![api_key.user.salt.as_str()],
                &alg.salting,
            );

            let delete = db
                .run(move |conn| diesel::delete(categoria).filter(id.eq(hash)).execute(conn))
                .await;

            match delete {
                Ok(_) => return Ok(Accepted(Some("Record found.".into()))),
                Err(_) => {
                    return Err(Unauthorized(Some(
                        "Failed to communicate to the database".into(),
                    )))
                }
            }
        }
        Err(_) => Err(Unauthorized(Some(
            "Failed to communicate to the database".into(),
        ))),
    }
}
