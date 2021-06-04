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

    pub async fn get(executor: &SqlitePool, id: i64) -> Result<Self> {
        sqlx::query_as!(Self, r#"select * from category where id = ?"#, id)
            .fetch_one(executor)
            .await
    }

    pub async fn tasks(&self, executor: &SqlitePool) -> Result<Vec<Task>> {
        sqlx::query_as!(Task, r#"select * from task where category_id = ?"#, self.id)
            .fetch_all(executor)
            .await
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub category_id: i64,
}

impl Task {
    pub async fn new(executor: &SqlitePool, title: &str, category: &Category) -> Result<Self> {
        sqlx::query_as!(
            Self,
            r#"insert into task(title, category_id) values (?, ?) returning id as "id!: i64", title as "title!: String", category_id as "category_id!: i64""#,
            title,
            category.id
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
