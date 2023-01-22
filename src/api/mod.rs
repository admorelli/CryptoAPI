use rocket::fairing::AdHoc;

pub mod category_item;
pub mod category_manager;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
        .mount("/api/category", routes![
            category_manager::index, 
            category_manager::get, 
            category_manager::add, 
            category_manager::del]
        )
        .mount("/api/data", routes![
            category_item::index,
            category_item::get,
            category_item::add,
            category_item::del,
            category_item::patch
        ])
    })
}