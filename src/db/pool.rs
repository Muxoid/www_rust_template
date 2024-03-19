use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn connect() -> Result<sqlx::Pool<sqlx::Postgres>, sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set in .env file");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
