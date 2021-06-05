use crate::Category;
use rocket::{get, response::Debug, routes, serde::json::Json, Route, State};
use sqlx::SqlitePool;

type Result<T, E = Debug<sqlx::Error>> = std::result::Result<T, E>;

#[get("/categories")]
pub async fn list_categories(db: &State<SqlitePool>) -> Result<Json<Vec<Category>>> {
    let categories = Category::all(&**db).await?;
    Ok(Json(categories))
}

#[get("/categories/<id>")]
pub async fn get_category(db: &State<SqlitePool>, id: i64) -> Result<Option<Json<Category>>> {
    let category = Category::get(&**db, id).await?;
    Ok(category.map(Json))
}

pub fn routes() -> Vec<Route> {
    routes![list_categories, get_category]
}
