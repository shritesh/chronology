use sqlx::{Result, SqlitePool};

#[derive(sqlx::FromRow, Debug)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

impl Task {
    pub async fn new(executor: &SqlitePool, title: &str) -> Result<Self> {
        sqlx::query_as!(
            Self,
            r#"insert into task(title) values (?) returning id as "id!: i64", title as "title!: String""#,
            title
        )
        .fetch_one(executor)
        .await
    }

    pub async fn fetch_all(executor: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as!(Self, r#"select * from task"#)
            .fetch_all(executor)
            .await
    }
}
