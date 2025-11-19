use std::env;

use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn db_client()->PgPool{
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL must be set in .env or environment");

    let conn=PgPoolOptions::new().connect(&database_url).await.expect("msg");

    conn

}