use rocket::fairing::AdHoc;
use rocket_okapi::openapi_get_routes;

pub mod category_item;
pub mod category_manager;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
            .mount(
                "/api/category",
                openapi_get_routes![
                    category_manager::index,
                    category_manager::get,
                    category_manager::add,
                    category_manager::del
                ],
            )
            .mount(
                "/api/data",
                openapi_get_routes![
                    category_item::index,
                    category_item::get,
                    category_item::add,
                    category_item::del,
                    category_item::patch
                ],
            )
    })
}
