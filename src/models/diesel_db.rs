//use diesel_migrations::embed_migrations;
//use rocket::{Rocket, Orbit};
use rocket::fairing::AdHoc;
//use rocket::serde::{Serialize, Deserialize};

use rocket_sync_db_pools::{database, diesel};

//#[database("development")]
//pub struct Db(diesel::SqliteConnection);

#[database("production")]
pub struct Db(diesel::PgConnection);


// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
//embed_migrations!();

// async fn run_migrations(rocket: Rocket<Orbit>) -> Result<(), Box<dyn Send + Sync + 'static>> {
//     todo!()
// }

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.attach(Db::fairing())
        //.attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}
