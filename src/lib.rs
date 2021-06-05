pub mod web;

use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Result, SqlitePool};

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Category {
    pub id: i64,
    pub title: String,
}

impl Category {
    pub async fn new(pool: &SqlitePool, title: &str) -> Result<Self> {
        sqlx::query_as!(
            Self,
            r#"insert into category(title) values (?) returning id as "id!: i64", title as "title!: String""#,
            title
        )
        .fetch_one(pool)
        .await
    }

    pub async fn all(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as!(Self, r#"select * from category"#)
            .fetch_all(pool)
            .await
    }

    pub async fn get(pool: &SqlitePool, id: i64) -> Result<Option<Self>> {
        sqlx::query_as!(Self, r#"select * from category where id = ?"#, id)
            .fetch_optional(pool)
            .await
    }

    pub async fn tasks(&self, pool: &SqlitePool) -> Result<Vec<Task>> {
        sqlx::query_as!(Task, r#"select * from task where category_id = ?"#, self.id)
            .fetch_all(pool)
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
    pub async fn new(pool: &SqlitePool, title: &str, category: &Category) -> Result<Self> {
        sqlx::query_as!(
            Self,
            r#"insert into task(title, category_id) values (?, ?) returning id as "id!: i64", title as "title!: String", category_id as "category_id!: i64""#,
            title,
            category.id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn all(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as!(Self, r#"select * from task"#)
            .fetch_all(pool)
            .await
    }

    pub async fn get(pool: &SqlitePool, id: i64) -> Result<Option<Self>> {
        sqlx::query_as!(Self, r#"select * from task where id = ?"#, id)
            .fetch_optional(pool)
            .await
    }
}

#[derive(sqlx::FromRow, Debug)]
pub struct Entry {
    pub id: i64,
    pub task_id: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub note: Option<String>,
}

impl Entry {
    pub async fn new_now(pool: &SqlitePool, task: &Task) -> Result<Self> {
        let now = Utc::now();
        sqlx::query_as!(
            Self,
            r#"insert into entry(task_id, start_time) values (?, ?) returning id as "id!: i64", task_id as "task_id!: i64", start_time as "start_time!: DateTime<Utc>", end_time as "end_time?: DateTime<Utc>", note as "note?: String""#,
            task.id,
            now
        )
        .fetch_one(pool)
        .await
    }

    pub async fn all(pool: &SqlitePool) -> Result<Vec<Self>> {
        sqlx::query_as!(Self, r#"select id, task_id, start_time as "start_time!: DateTime<Utc>", end_time as "end_time?: DateTime<Utc>", note from entry"#)
            .fetch_all(pool)
            .await
    }
}
