mod task;

use sqlx::{Result, SqlitePool};
use task::Task;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite:chronology.db").await?;

    let _task = Task::new(&pool, "Build this app").await?;

    for task in Task::fetch_all(&pool).await? {
        dbg!(task);
    }

    Ok(())
}
