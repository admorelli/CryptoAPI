use diesel::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use rocket::response::status::{Accepted, Created, NotFound, Unauthorized};
use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::models::categoria::Categoria;
use crate::models::diesel_db::Db;
use crate::models::hash::{hash::dsl::*, Hash};
use crate::security::auth_key::ApiKey;

#[openapi]
#[get("/<category>")]
pub async fn index(api_key: ApiKey, db: Db, category: String) -> Result<Json<Vec<Hash>>, String> {
    let categoria = Categoria::from_db(&category, &db, &api_key, None).await.unwrap();

    let hashes = db
        .run(move |conn| {
            use crate::models::hash::hash::dsl::*;
            hash.filter(
                owner.eq_any(
                    categoria
                        .clone()
                        .into_iter()
                        .map(|c| c.id)
                        .collect::<Vec<_>>(),
                ),
            )
                .load::<Hash>(conn)
        })
        .await.or_else(|e| Err(e.to_string()))?.into_iter().collect::<Vec<_>>();
    Ok(Json(hashes))
}

#[openapi]
#[get("/<category>/<key>/<data>")]
pub async fn get(
    api_key: ApiKey,
    db: Db,
    category: String,
    key: String,
    data: String,
) -> Result<Accepted<String>, NotFound<String>> {
    let hashes = Hash::from_db((&key, &data), &category, &db, &api_key, None).await.expect("Hash not found");
    let alg = hashes.0;
    let hash_model = hashes.2;
    let hashed_data_string = alg.apply(
        &data,
        &vec![
            &api_key.user.salt.as_str(),
            &hashes.1.salt,
            &hash_model.salt.as_str(),
        ],
    );

    if hash_model.hashed_data == hashed_data_string {
        return Ok(Accepted(Some("Record found!".into())));
    } else {
        return Err(NotFound("Record not found or invalid".into()));
    }
}

#[openapi]
#[post("/<category>/<key>/<data>")]
pub async fn add(
    api_key: ApiKey,
    db: Db,
    category: String,
    key: String,
    data: String,
) -> Result<Created<String>, Unauthorized<String>> {
    let category_models = Categoria::from_db(&category, &db, &api_key, Some(true)).await.expect("Category not found.");
    let category_model = category_models.first().expect("Category not found.");

    let alg = api_key.algorithms.first().expect("No valid algorithm Found");
    let generated_salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
    let category_salt = &category_model.clone().salt;

    let hash_id = alg.apply(
        key.as_str(),
        &vec![api_key.user.salt.as_str(), category_salt.as_str()],
    );
    let hashed_data_string = alg.apply(
        data.as_str(),
        &vec![
            &api_key.user.salt.as_str(),
            &category_model.salt.as_str(),
            &generated_salt.as_str(),
        ],
    );
    let category_id = category_model.id.clone();
    let result = db
        .run(move |conn| {
            diesel::insert_into(hash)
                .values((
                    id.eq(hash_id.as_str()),
                    is_unsafe.eq(false),
                    salt.eq(generated_salt.as_str()),
                    hashed_data.eq(hashed_data_string),
                    owner.eq(category_id),
                ))
                .execute(conn)
        })
        .await;

    match result {
        Ok(_) => Ok(Created::new("Success")),
        Err(_) => Err(Unauthorized(Some("Failed to insert".into()))),
    }
}

#[openapi]
#[delete("/<category>/<key>/<data>")]
pub async fn del(
    api_key: ApiKey,
    db: Db,
    category: String,
    key: String,
    data: String,
) -> Result<Accepted<String>, Unauthorized<String>> {
    let hash_id = Hash::from_db((&key, &data), &category, &db, &api_key, None).await.or_else(|e| Err(Unauthorized(Some(format!("{}", e)))))?.2;
    let result = db.run(move |conn| {
        use diesel::dsl::delete;
        use diesel::prelude::*;
        delete(hash)
            .filter(id.eq(hash_id.id))
            .execute(conn)
    }).await.or_else(|e| Err(Unauthorized(Some(format!("{}", e)))))?;
    if result == 0 {
        Ok(Accepted(Some("Success".into())))
    } else {
        Err(Unauthorized(Some("Failed to insert".into())))
    }
}

#[openapi]
#[patch("/<_category>/<_key>/<_data>")]
pub fn patch(_api_key: ApiKey, _db: Db, _category: String, _key: String, _data: String) -> String {
    todo!()
}
