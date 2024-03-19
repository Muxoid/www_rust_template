use crate::db::models::User;
use crate::db::pool::connect;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::Error;

pub async fn get_users() -> Result<Vec<User>, Error> {
    let db = connect().await?;
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(&db)
        .await?;
    Ok(users)
}
pub async fn create_user(name: String, email: String, password: String) -> Result<(), Error> {
    let db = connect().await?;

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash_password")
        .to_string();

    sqlx::query!(
        "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3)",
        name,
        email,
        password_hash
    )
    .execute(&db)
    .await?;
    Ok(())
}
