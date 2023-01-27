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

mod openapi {
    use rocket::fairing::{AdHoc};
    use rocket_okapi::{swagger_ui::*, mount_endpoints_and_merged_docs};

    pub fn get_docs() -> SwaggerUIConfig {
        SwaggerUIConfig {
            url: "/v1/openapi.json".to_string(),
            ..Default::default()
        }
    }

    pub fn stage() -> AdHoc {
        AdHoc::on_ignite("OpenAPI Documentation", |mut rocket| async {
            let openapi_settings = rocket_okapi::settings::OpenApiSettings::default();
            mount_endpoints_and_merged_docs! {
                rocket, "/v1".to_owned(), openapi_settings,
                    "/api/category" => crate::api::category_manager::get_routes_and_docs(&openapi_settings),
                    "/api/data" => crate::api::category_item::get_routes_and_docs(&openapi_settings),
            }
            rocket
                .mount("/swagger", make_swagger_ui(&get_docs()))
        })
    }
}

#[launch]
async fn rocket() -> _ {
    let prometheus = PrometheusMetrics::new();
    rocket::build()
        .attach(prometheus.clone())
        .attach(models::diesel_db::stage())
        .attach(pages::stage())
        //.attach(api::stage())
        .attach(openapi::stage())
        .mount("/metrics", prometheus)
}
