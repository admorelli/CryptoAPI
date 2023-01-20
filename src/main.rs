#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#[macro_use] extern crate rocket;

use database::bb8_util::{DbTypes, SurrealdbMan, SurrealdbBackend};
use rocket_prometheus::PrometheusMetrics;

mod api;
mod pages;
mod database;
mod security;
mod models;




#[launch]
async fn rocket() -> _ {
    let prometheus = PrometheusMetrics::new();
    let tp = DbTypes::Surrealdb(SurrealdbMan{
        backend: SurrealdbBackend::File{
            file: "tempdb.db",
            ns_name: "Teste", 
            db_name: "Teste"
        }
    });
    let pool = database::bb8_util::configure_bb8pool(tp).await.unwrap();
    rocket::build()
    .manage(pool)
    .attach(prometheus.clone())
    .mount("/metrics", prometheus)
    .mount("/", routes![pages::index::index, pages::index::protected])
    .mount("/api/category", routes![
        api::category_manager::index, 
        api::category_manager::get, 
        api::category_manager::add, 
        api::category_manager::del]
    )
    .mount("/api/data", routes![
        api::category_item::index,
        api::category_item::get,
        api::category_item::add,
        api::category_item::del,
        api::category_item::patch
    ])
    //.mount("/", routes![index::protected])
}