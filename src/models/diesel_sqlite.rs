use diesel_migrations::{embed_migrations};
use rocket::fairing::AdHoc;
use rocket::response::{Debug};
use rocket::serde::{Serialize, Deserialize};

use rocket_sync_db_pools::{diesel, database};

use self::diesel::prelude::*;

#[database("development")]
pub struct Db(diesel::SqliteConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name="posts"]
struct Post {
    #[serde(skip_deserializing)]
    id: Option<i32>,
    title: String,
    text: String,
    #[serde(skip_deserializing)]
    published: bool,
}

table! {
    posts (id) {
        id -> Nullable<Integer>,
        title -> Text,
        text -> Text,
        published -> Bool,
    }
}

// This macro from `diesel_migrations` defines an `embedded_migrations` module
// containing a function named `run`. This allows the example to be run and
// tested without any outside setup of the database.
embed_migrations!();

async fn run_migrations(connection: Db) -> Result<(), Box<dyn Send + Sync + 'static>> {

    
    connection.run(|c| embedded_migrations::run(c)).await.expect("diesel migrations");

    Ok(())
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket.attach(Db::fairing())
            //.attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
    })
}