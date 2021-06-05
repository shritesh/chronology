use chronology::Category;
use rocket::{
    fairing::{self, AdHoc},
    get,
    response::Debug,
    serde::json::Json,
    Build, Rocket, State,
};
use sqlx::SqlitePool;

type Result<T, E = Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/categories")]
async fn list_categories(db: &State<SqlitePool>) -> Result<Json<Vec<Category>>> {
    let categories = Category::all(&**db).await?;
    Ok(Json(categories))
}

#[get("/categories/<id>")]
async fn get_category(db: &State<SqlitePool>, id: i64) -> Result<Option<Json<Category>>> {
    let category = Category::get(&**db, id).await?;
    Ok(category.map(Json))
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::try_on_ignite("SQLx connection", init_database))
        .mount("/", rocket::routes![list_categories, get_category])
}

async fn init_database(rocket: Rocket<Build>) -> fairing::Result {
    match SqlitePool::connect("sqlite:chronology.db").await {
        Ok(pool) => Ok(rocket.manage(pool)),
        Err(e) => {
            rocket::error!("Failed to connect to database: {}", e);
            Err(rocket)
        }
    }
}
