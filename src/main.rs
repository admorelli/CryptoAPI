#[macro_use] extern crate rocket;
//#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

use rocket_prometheus::PrometheusMetrics;

mod api;
mod pages;
mod security;
mod models;

#[launch]
async fn rocket() -> _ {
    let prometheus = PrometheusMetrics::new();
    rocket::build()
    .attach(prometheus.clone())
    .attach(models::diesel_sqlite::stage())
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