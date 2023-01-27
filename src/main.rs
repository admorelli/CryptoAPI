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
    use rocket::fairing::AdHoc;
    use rocket_okapi::{rapidoc::*, swagger_ui::*, settings::UrlObject};

    pub fn get_docs() -> SwaggerUIConfig {
        SwaggerUIConfig {
            urls: vec![
                UrlObject::new("Category", "/api/category/openapi.json"),
                UrlObject::new("Data", "/api/data/openapi.json"),
            ],
            ..Default::default()
        }
    }

    pub fn stage() -> AdHoc {
        AdHoc::on_ignite("OpenAPI Documentation", |rocket| async {
            rocket
                .mount("/swagger", make_swagger_ui(&get_docs()))
                .attach(AdHoc::on_liftoff("Migrations", |_| {
                    Box::pin(async move {
                        let doc = make_rapidoc(&RapiDocConfig {
                            general: GeneralConfig {
                                spec_urls: vec![UrlObject::new("General", "../../openapi.json")],
                                ..Default::default()
                            },
                            hide_show: HideShowConfig {
                                allow_spec_url_load: false,
                                allow_spec_file_load: false,
                                ..Default::default()
                            },
                            ..Default::default()
                        }).into();
                        if doc.len() > 0 {
                            info!("openapi.json successfully generated.")
                        } else {
                            info!("failed to generate openapi.json.")
                        }
                    })
                }))
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
        .attach(api::stage())
        .attach(openapi::stage())
        .mount("/metrics", prometheus)

    //.mount("/", routes![index::protected])
}
