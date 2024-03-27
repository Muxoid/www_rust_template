use actix_web::{
    get,
    http::StatusCode,
    post,
    web::{self},
    App, Error, HttpResponse, HttpServer, Responder,
};
use askama::Template;
use serde::Deserialize;
use sqlx::Error as SqlxError; // Make sure this import is correct

use crate::auth;
use crate::db;

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
    username: String,
    password: String,
}

#[post("/login")]
async fn login(form: web::Form<LoginForm>) -> impl Responder {
    let token = match auth::jwt::generate_jwt() {
        Ok(t) => t,
        Err(_) => return HttpResponse::InternalServerError().body("JWT Generation Failed"),
    };
    let template = IndexTemplate;
    match template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(_) => HttpResponse::InternalServerError().body("Internal Server Error"),
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
async fn register_user(form: web::Form<RegisterForm>) -> Result<impl Responder, Error> {
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

        Err(e) => {
            if let SqlxError::Database(db_err) = &e {
                if db_err.code().map_or(false, |code| code == "23505") {
                    // Handle the duplicate username error
                    return Ok(HttpResponse::BadRequest()
                        .content_type("text/html")
                        .body("Username already exists."));
                }
            }
            // For other errors, convert them into an internal server error
            Err(actix_web::error::ErrorInternalServerError(e))
        }
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(index)
        .service(users)
        .service(login_form)
        .service(register_user)
        .service(register_form);
}
