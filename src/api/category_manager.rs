use rocket::serde::json::Json;
use rocket::{response::status::*};
use diesel::prelude::*;

use crate::models::categoria::Categoria;
use crate::models::diesel_sqlite::Db;
use crate::security::auth_key::ApiKey;

#[get("/")]
pub async fn index(key: ApiKey, db: Db) -> Result<Json<Vec<Categoria>>, String>{
    use crate::models::categoria::categoria::dsl::*;
    use diesel::QueryDsl;

    let result = db.run(move |conn| {
        categoria
        .filter(owner.eq(key.user.id))
        .load::<Categoria>(conn)
    }).await;
            
    match result {
        Ok(categorias) =>{
            Ok(Json(categorias))
        },
        Err(e) => Err(e.to_string())
    }
    //correct message
    //Ok("Endpoint to manage categorias")
}

#[get("/<key>")]
pub async fn get(api_key: ApiKey, db: Db, key: String) -> Result<Accepted<String>, NotFound<String>>{
    use crate::models::categoria::categoria::dsl::*;
    
    if api_key.algorithms.len() > 0 {
        
        let result = db.run(move |conn|{
            categoria
                .filter(owner.eq(api_key.user.id))
                .load::<Categoria>(conn)
        }).await;

        match result{
            Ok(res) =>{
                
                let alg = api_key.algorithms.first().unwrap();
        
                for cat in res{
                    let hash = alg.crypto.apply(key.as_str(), &vec![api_key.user.salt.as_str(), cat.salt.as_str()], &alg.salting);
                    if cat.id == hash{
                        return Ok(Accepted(Some("Record found.".into())));
                    }
                }
                Err(NotFound("Record not found.".into()))
            },
            Err(_) => Err(NotFound("Failed to communicate to the database".into()))
        }

    }else{
        Err(NotFound("No Algorithms to choose from".into()))
    }
}

#[post("/<key>")]
pub async fn add(api_key: ApiKey, db: Db, key: String) -> Result<Created<String>, Unauthorized<String>>{
    use crate::models::categoria::categoria::dsl::*;
    use rand::distributions::{Alphanumeric, DistString};
    

    if api_key.algorithms.len() > 0 {
        
        let mut generated_salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);


        let result = db.run(move |conn| {
            
            let categorias = categoria
                .filter(owner.eq(api_key.user.id))
                .load::<Categoria>(conn)
                .unwrap();
            
            let alg = api_key.algorithms.first().unwrap();
        
            for cat in categorias{
                let hash = alg.crypto.apply(key.as_str(), &vec![api_key.user.salt.as_str(), cat.salt.as_str()], &alg.salting);
                if cat.id == hash{
                    return Err(diesel::result::Error::AlreadyInTransaction);
                }
            }
            
            let mut hash = alg.crypto.apply(key.as_str(), &vec![api_key.user.salt.as_str(), generated_salt.as_str()], &alg.salting);

            while match categoria
                .filter(id.eq(hash.as_str()))
                .count()
                .execute(conn)
                {
                    Ok(u) => u > 0,
                    Err(e) => return Err(e)
                }
            {
                generated_salt = Alphanumeric.sample_string(&mut rand::thread_rng(), 16);
                hash = alg.crypto.apply(key.as_str(), &vec![api_key.user.salt.as_str(), generated_salt.as_str()], &alg.salting);
            }

            
            diesel::insert_into(categoria)
            .values((
                id.eq(hash.as_str()),
                is_unsafe.eq(false),
                salt.eq(generated_salt.as_str()),
                owner.eq(api_key.user.id)
            ))
            .execute(conn)
        }).await;

        match result{
            Ok(_) => Ok(Created::new("Success")),
            Err(_) => Err(Unauthorized(Some("Failed to insert".into())))
        }
    }
    else{
        Err(Unauthorized(Some("No Algorithms to choose from".to_string())))
    }
}

#[delete("/<key>")]
pub async fn del(api_key: ApiKey, db: Db, key: String) -> Result<Accepted<String>, Unauthorized<String>>{
    use crate::models::categoria::categoria::dsl::*;
    
    if api_key.algorithms.len() > 0 {
        
        let result = db.run(move |conn|{
            categoria
                .filter(owner.eq(api_key.user.id))
                .load::<Categoria>(conn)
        }).await;

        match result{
            Ok(res) =>{
                
                let alg = api_key.algorithms.first().unwrap();
        
                for cat in res{
                    let hash = alg.crypto.apply(key.as_str(), &vec![api_key.user.salt.as_str(), cat.salt.as_str()], &alg.salting);
                    if cat.id == hash{
                        let delete = db.run(move |conn|{
                            diesel::delete(categoria)  
                                .filter(id.eq(hash))
                                .execute(conn)
                        }).await;

                        match delete{
                            Ok(_) => return Ok(Accepted(Some("Record found.".into()))),
                            Err(_) => return Err(Unauthorized(Some("Failed to communicate to the database".into())))
                        };
                    }
                }
                Err(Unauthorized(Some("Record not found.".into())))
            },
            Err(_) => Err(Unauthorized(Some("Failed to communicate to the database".into())))
        }

    }else{
        Err(Unauthorized(Some("No Algorithms to choose from".into())))
    }
}