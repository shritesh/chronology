use chronology::web;
use rocket::{
    http::{ContentType, Status},
    local::asynchronous::Client,
};
use sqlx::SqlitePool;

async fn setup() -> Client {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();

    let rocket = rocket::build().mount("/", web::routes()).manage(pool);

    Client::tracked(rocket).await.unwrap()
}

#[rocket::async_test]
async fn get_categories() {
    let client = setup().await;

    let res = client.get("/categories").dispatch().await;
    assert_eq!(res.status(), Status::Ok);
    assert_eq!(res.content_type(), Some(ContentType::JSON));
}
