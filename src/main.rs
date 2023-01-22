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
    .attach(pages::stage())
    .attach(api::stage())
    .mount("/metrics", prometheus)
    //.mount("/", routes![index::protected])
}