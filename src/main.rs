use chronology::Entry;
use sqlx::{Result, SqlitePool};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = SqlitePool::connect("sqlite:chronology.db").await?;

    for entry in Entry::all(&pool).await? {
        dbg!(entry);
    }

    Ok(())
}
