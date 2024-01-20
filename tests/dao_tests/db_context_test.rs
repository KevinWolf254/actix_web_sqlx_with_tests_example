use std::env;

use actix_web_sqlx_tests_example::dao::Database;
use dotenvy::dotenv;

#[sqlx::test]
async fn new_returns_db_context_when_url_is_valid() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL has not been set!");

    Database::new(&database_url, 5).await;
}