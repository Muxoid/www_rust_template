use crate::db::models::User;
use crate::db::pool::connect;
use crate::utils::error::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

#[tracing::instrument]
pub async fn get_users() -> Result<Vec<User>, AppError> {
    let db = connect().await?;
    let users = sqlx::query_as!(User, "SELECT id, name, email, password_hash FROM users")
        .fetch_all(&db)
        .await?;
    Ok(users)
}
#[tracing::instrument(skip(password))]
pub async fn create_user(name: String, email: String, password: String) -> Result<(), AppError> {
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

#[tracing::instrument]
pub async fn get_one_user(username: &String, email: &String) -> Result<Option<User>, AppError> {
    let db = connect().await?;
    let user_result = sqlx::query_as!(
        User,
        "SELECT id, name, email, password_hash FROM users where name = $1 or email = $2",
        username,
        email
    )
    .fetch_one(&db)
    .await;

    match user_result {
        Ok(user) => Ok(Some(user)), // If a user is found, wrap it in Some and return
        Err(e) => match e {
            sqlx::Error::RowNotFound => Ok(None), // If no user is found, return None
            _ => Err(e.into()), // For any other error, convert it into your AppError type and return it
        },
    }
}

#[tracing::instrument]
pub async fn get_user_by_id(userID: i32) -> Result<Option<User>, AppError> {
    let db = connect().await?;
    let user_result = sqlx::query_as!(
        User,
        "SELECT id, name, email, password_hash FROM users where ID = $1 ",
        userID
    )
    .fetch_one(&db)
    .await;

    match user_result {
        Ok(user) => Ok(Some(user)), // If a user is found, wrap it in Some and return
        Err(e) => match e {
            sqlx::Error::RowNotFound => Ok(None), // If no user is found, return None
            _ => Err(e.into()), // For any other error, convert it into your AppError type and return it
        },
    }
}

#[tracing::instrument]
pub async fn get_one_user_by_username_or_email(
    username_or_email: &String,
) -> Result<Option<User>, AppError> {
    let db = connect().await?;
    let user_result = sqlx::query_as!(
        User,
        "SELECT id, name, email, password_hash FROM users where name = $1 or email = $1",
        username_or_email
    )
    .fetch_one(&db)
    .await;

    match user_result {
        Ok(user) => Ok(Some(user)), // If a user is found, wrap it in Some and return
        Err(e) => match e {
            sqlx::Error::RowNotFound => Ok(None), // If no user is found, return None
            _ => Err(e.into()), // For any other error, convert it into your AppError type and return it
        },
    }
}
