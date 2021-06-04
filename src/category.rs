use sqlx::{Result, SqlitePool};

#[derive(sqlx::FromRow, Debug)]
pub struct Category {
    id: i64,
    title: String,
}

impl Category {
    pub async fn new(executor: &SqlitePool, title: &str) -> Result<Self> {
        sqlx::query_as!(
            Self,
            r#"insert into category(title) values (?) returning id as "id!: i64", title as "title!: String""#,
            title
        )
        .fetch_one(executor)
        .await
    }

    pub async fn fetch_all(executor: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as!(Self, r#"select * from category"#)
            .fetch_all(executor)
            .await
    }
}
