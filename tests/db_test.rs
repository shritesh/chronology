use chronology::{self, Category};
use sqlx::{Result, SqlitePool};

async fn setup() -> Result<SqlitePool> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_categories() -> Result<()> {
    let pool = setup().await?;

    let title = "Test Category";

    let category = Category::new(&pool, title).await?;
    assert_eq!(category.title, title);

    let categories = Category::all(&pool).await?;
    assert_eq!(categories.len(), 1);

    let category_ = Category::get(&pool, category.id)
        .await?
        .expect("Category not found");
    assert_eq!(category_.title, title);

    Ok(())
}
