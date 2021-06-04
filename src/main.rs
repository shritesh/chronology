mod category;
mod task;

use category::Category;
use sqlx::{Result, SqlitePool};
use task::Task;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite:chronology.db").await?;

    let _category = Category::new(&pool, "Programming").await?;

    for category in Category::fetch_all(&pool).await? {
        dbg!(category);
    }

    let _task = Task::new(&pool, "Build this app").await?;

    for task in Task::fetch_all(&pool).await? {
        dbg!(task);
    }

    Ok(())
}
