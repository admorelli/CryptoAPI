#[macro_use]
extern crate rocket;
//#[macro_use] extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

use rocket_prometheus::PrometheusMetrics;

mod api;
mod models;
mod pages;
mod security;

#[launch]
async fn rocket() -> _ {
    let prometheus = PrometheusMetrics::new();
    rocket::build()
        .attach(prometheus.clone())
        .attach(models::diesel_db::stage())
        .attach(pages::stage())
        .attach(api::stage())
        .mount("/metrics", prometheus)
    //.mount("/", routes![index::protected])
}
