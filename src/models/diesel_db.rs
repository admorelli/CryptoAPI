use rocket::fairing::AdHoc;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::gen::OpenApiGenerator;

use rocket_sync_db_pools::{database, diesel};

#[database("application")]
#[derive(MigrationConnection)]
pub struct Db(diesel::PgConnection);

impl<'r> OpenApiFromRequest<'r> for Db {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}

#[allow(dead_code)]
mod embedded_migrations {
    use diesel_migrations::EmbedMigrations;

    #[derive(EmbedMigrations)]
    struct Dummy;
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
        rocket
            .attach(Db::fairing())
            .attach(AdHoc::on_liftoff("Migrations", |rocket| {
                Box::pin(async move {
                    let db = Db::get_one(&rocket).await.expect("Failed to connect");

                    db.run(move |conn| {
                        match embedded_migrations::run(conn) {
                            Ok(_) => info!("Migration successfull"),
                            Err(e) => info!("{}", e),
                        };
                    })
                      .await;
                })
            }))
    })
}
