use diesel::prelude::*;
use rand::distributions::{Alphanumeric, DistString};
use rocket::response::status::{Accepted, Created, NotFound, Unauthorized};
use rocket::serde::json::Json;

use crate::models::categoria::Categoria;
use crate::models::diesel_sqlite::Db;
use crate::models::hash::{hash::dsl::*, Hash};
use crate::security::auth_key::ApiKey;

#[get("/<category>")]
pub async fn index(api_key: ApiKey, db: Db, category: String) -> Result<Json<Vec<Hash>>, String> {
    let alg = api_key.algorithms.first().unwrap();
    let category_id = alg.apply(category.as_str(), &vec![api_key.user.salt.as_str()]);
    let categoria = match db
        .run(move |conn| {
            use crate::models::categoria::categoria::dsl::*;
            categoria
                .filter(owner.eq(api_key.user.id))
                .filter(id.eq(category_id))
                .first::<Categoria>(conn)
        })
        .await
    {
        Ok(c) => c,
        Err(e) => return Err(e.to_string()),
    };

    let hashes = match db
        .run(move |conn| {
            use crate::models::hash::hash::dsl::*;
            hash.filter(owner.eq(categoria.id)).load::<Hash>(conn)
        })
        .await
    {
        Ok(h) => h.into_iter().collect::<Vec<_>>(),
        Err(e) => return Err(e.to_string()),
    };
    Ok(Json(hashes))
}

#[get("/<category>/<key>/<data>")]
pub async fn get(
    api_key: ApiKey,
    db: Db,
    category: String,
    key: String,
    data: String,
) -> Result<Accepted<String>, NotFound<String>> {
    let alg = api_key.algorithms.first().unwrap();
    let category_id = alg.apply(category.as_str(), &vec![api_key.user.salt.as_str()]);

    let category_model = match db
        .run(move |conn| {
            use crate::models::categoria::categoria::dsl::*;
            categoria
                .filter(owner.eq(api_key.user.id))
                .filter(id.eq(category_id))
                .first::<Categoria>(conn)
        })
        .await
    {
        Ok(c) => c,
        Err(e) => return Err(NotFound(e.to_string())),
    };

    let hash_id = alg.apply(
        key.as_str(),
        &vec![api_key.user.salt.as_str(), category_model.salt.as_str()],
    );

    let hash_model = match db
        .run(move |conn| {
            hash.filter(owner.eq(category_model.id))
                .filter(id.eq(hash_id))
                .first::<Hash>(conn)
        })
        .await
    {
        Ok(h) => h,
        Err(e) => return Err(NotFound(e.to_string())),
    };

    let hashed_data_string = alg.apply(
        data.as_str(),
        &vec![
            api_key.user.salt.as_str(),
            category_model.salt.as_str(),
            hash_model.salt.as_str(),
        ],
    );
    if hashed_data_string == hash_model.hashed_data {
        Ok(Accepted(Some("found valid data".into())))
    } else {
        Err(NotFound("Record not found or invalid".into()))
    }
}

#[post("/<category>/<key>/<data>")]
pub async fn add(
    api_key: ApiKey,
    db: Db,
    category: String,
    key: String,
    data: String,
) -> Result<Created<String>, Unauthorized<String>> {
    let alg = api_key.algorithms.first().unwrap();
    let category_id = alg.apply(category.as_str(), &vec![api_key.user.salt.as_str()]);
    let generated_salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let category_model = match db
        .run(move |conn| {
            use crate::models::categoria::categoria::dsl::*;
            categoria
                .filter(owner.eq(api_key.user.id))
                .filter(id.eq(category_id))
                .first::<Categoria>(conn)
        })
        .await
    {
        Ok(c) => c,
        Err(e) => return Err(Unauthorized(Some(e.to_string()))),
    };

    let hash_id = alg.apply(
        key.as_str(),
        &vec![api_key.user.salt.as_str(), category_model.salt.as_str()],
    );
    let hashed_data_string = alg.apply(
        data.as_str(),
        &vec![
            api_key.user.salt.as_str(),
            category_model.salt.as_str(),
            generated_salt.as_str(),
        ],
    );

    let result = db
        .run(move |conn| {
            diesel::insert_into(hash)
                .values((
                    id.eq(hash_id.as_str()),
                    is_unsafe.eq(false),
                    salt.eq(generated_salt.as_str()),
                    hashed_data.eq(hashed_data_string),
                    owner.eq(category_model.id),
                ))
                .execute(conn)
        })
        .await;

    match result {
        Ok(_) => Ok(Created::new("Success")),
        Err(_) => Err(Unauthorized(Some("Failed to insert".into()))),
    }
}

#[delete("/<category>/<key>/<data>")]
pub async fn del(
    api_key: ApiKey,
    db: Db,
    category: String,
    key: String,
    data: String,
) -> Result<Accepted<String>, Unauthorized<String>> {
    let alg = api_key.algorithms.first().unwrap();
    let category_id = alg.apply(category.as_str(), &vec![api_key.user.salt.as_str()]);
    let generated_salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);

    let category_model = match db
        .run(move |conn| {
            use crate::models::categoria::categoria::dsl::*;
            categoria
                .filter(owner.eq(api_key.user.id))
                .filter(id.eq(category_id))
                .first::<Categoria>(conn)
        })
        .await
    {
        Ok(c) => c,
        Err(e) => return Err(Unauthorized(Some(e.to_string()))),
    };

    let hash_id = alg.apply(
        key.as_str(),
        &vec![api_key.user.salt.as_str(), category_model.salt.as_str()],
    );
    let hashed_data_string = alg.apply(
        data.as_str(),
        &vec![
            api_key.user.salt.as_str(),
            category_model.salt.as_str(),
            generated_salt.as_str(),
        ],
    );

    let result = db
        .run(move |conn| {
            diesel::delete(hash)
                .filter(id.eq(hash_id.as_str()))
                .filter(owner.eq(category_model.id))
                .filter(hashed_data.eq(hashed_data_string))
                .execute(conn)
        })
        .await;

    match result {
        Ok(_) => Ok(Accepted(Some("Success".into()))),
        Err(_) => Err(Unauthorized(Some("Failed to insert".into()))),
    }
}

#[patch("/<_category>/<_key>/<_data>")]
pub fn patch(_api_key: ApiKey, _db: Db, _category: String, _key: String, _data: String) -> String {
    todo!()
}
