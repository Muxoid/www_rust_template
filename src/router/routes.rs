use crate::auth;
use crate::db;
use crate::db::models::User;
use crate::utils::error::{AppError, AppErrorType};
use actix_web::body::MessageBody;
use actix_web::{
    cookie,
    cookie::Cookie,
    get,
    http::StatusCode,
    post,
    web::{self},
    App, Error, HttpResponse, HttpServer, Responder,
};
use anyhow::{Context, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use askama::Template;
use auth::jwt::Claims;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

#[get("/login")]
async fn login_form() -> impl Responder {
    let template = LoginTemplate;
    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[derive(Deserialize)]
struct LoginForm {
    email_or_username: String,
    password: String,
}

#[post("/login")]
async fn login(form: web::Form<LoginForm>) -> Result<impl Responder, AppError> {
    let template = LoginTemplate;
    let username_or_email = &form.email_or_username;
    let user_opt = db::query::get_one_user_by_username_or_email(username_or_email).await?;
    let user = user_opt.ok_or(AppError::from(AppErrorType::UserNotFound))?;

    let passwd = &form.password;
    let password_hash = user.password_hash;
    let parsed_hash = PasswordHash::new(&password_hash).expect("Parsing Password Failed");
    let passwd_check: bool = Argon2::default()
        .verify_password(&passwd.as_bytes(), &parsed_hash)
        .is_ok();

    if passwd_check {
        let redirect_url = format!("/u/{}", user.name);
        let token = auth::jwt::create_token(user.name)?;
        let cookie = Cookie::build("auth_token", token)
            .secure(true)
            .http_only(true)
            .same_site(cookie::SameSite::Strict)
            .path("/")
            .finish();
        match template.render() {
            Ok(_) => Ok(HttpResponse::SeeOther()
                .insert_header(("Set-Cookie", cookie.to_string()))
                .insert_header(("Location", redirect_url))
                .finish()),
            Err(_) => Ok(HttpResponse::InternalServerError().body("Internal Server Error")),
        }
    } else {
        Err(AppErrorType::IncorrectLogin)?
    }
}

#[derive(Template)]
#[template(path = "profile.html")]
struct UserProfileTemplate {
    username: String,
    email: String,
}

#[derive(Template)]
#[template(path = "forbidden.html")]
struct ForbiddenTemplate;

#[get("/u/{username}")]
async fn profile(
    claims: Claims,
    username_path: web::Path<String>,
) -> Result<impl Responder, AppError> {
    let username = claims.sub;
    let user_opt = db::query::get_one_user_by_username_or_email(&username).await?;

    match user_opt {
        Some(user) if username == username_path.to_string() => {
            let template = UserProfileTemplate {
                username: user.name,
                email: user.email,
                // Populate other fields as needed
            };

            let rendered = template
                .render()
                .map_err(|_| AppErrorType::TemplateRenderError)?;
            Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
        }
        Some(_) => Err(AppError::from(AppErrorType::IncorrectLogin)),
        None => Err(AppError::from(AppErrorType::IncorrectLogin)),
    }
}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate;

#[get("/register")]
async fn register_form() -> impl Responder {
    let template = RegisterTemplate;
    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
    }
}

#[derive(Deserialize)]
struct RegisterForm {
    username: String,
    email: String,
    password: String,
}

#[post("/register")]
async fn register_user(form: web::Form<RegisterForm>) -> Result<impl Responder, AppError> {
    let username = &form.username;
    let email = &form.email;
    let user_opt = db::query::get_one_user(username, email).await?;
    if user_opt.is_none() {
        match db::query::create_user(
            form.username.clone(),
            form.email.clone(),
            form.password.clone(),
        )
        .await
        {
            Ok(_) => Ok(HttpResponse::SeeOther()
                .append_header(("Location", "/users"))
                .finish()),
            Err(_) => Err(AppErrorType::ErrorDB)?,
        }
    } else {
        Err(AppErrorType::UserAlreadyExists)?
    }
}

#[derive(Template)]
#[template(path = "user_list.html")]
struct UserListTemplate {
    users: Vec<db::models::User>,
}

#[get("/users")]
async fn users() -> impl Responder {
    let users = db::query::get_users().await.expect("Could not get users!");
    let template = UserListTemplate { users };
    let rendered = template.render().expect("Could not render /users!");
    HttpResponse::Ok().body(rendered)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[get("/")]
async fn index() -> impl Responder {
    let template = IndexTemplate;
    let rendered = template.render().expect("Could not render /users!");
    HttpResponse::Ok().body(rendered)
}

#[derive(Template)]
#[template(path = "unauth.html")]
struct UnauthTemplate;

#[get("/unauth")]
async fn unauth() -> impl Responder {
    let template = UnauthTemplate;
    let rendered = template.render().expect("Could not render /users!");
    HttpResponse::Ok().body(rendered)
}

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFOundTemplate;

async fn not_found() -> impl Responder {
    let template = NotFOundTemplate;
    let rendered = template.render().expect("Could not render /users!");
    HttpResponse::Ok().body(rendered)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(users)
        .service(login_form)
        .service(login)
        .service(register_user)
        .service(register_form)
        .service(profile)
        .service(unauth)
        .default_service(
            web::route().to(not_found), // Use the custom not_found handler for unmatched routes
        );
}
