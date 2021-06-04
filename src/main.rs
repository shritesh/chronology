use chronology::Category;
use sqlx::{Result, SqlitePool};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite:chronology.db").await?;

    let category = Category::get(&pool, 1).await?;

    dbg!(&category);

    for task in category.tasks(&pool).await? {
        dbg!(task);
    }

    Ok(())
}
