use chronology::web;
use rocket::{
    fairing::{self, AdHoc},
    Build, Rocket,
};
use sqlx::SqlitePool;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("Database", db))
        .mount("/", web::routes())
}

async fn db(rocket: Rocket<Build>) -> fairing::Result {
    match SqlitePool::connect("sqlite:chronology.db").await {
        Ok(pool) => Ok(rocket.manage(pool)),
        Err(e) => {
            rocket::error!("Failed to connect to database: {}", e);
            Err(rocket)
        }
    }
}
