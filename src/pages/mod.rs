use rocket::fairing::AdHoc;

pub mod index;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.mount("/", routes![index::index, index::protected])
    })
}
