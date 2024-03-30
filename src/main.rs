use actix_web::{middleware, App, HttpServer};
mod auth;
mod db;
mod router;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Initialize the logger
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .configure(router::routes::config)
        // Add more routes for creating, updating, and deleting users
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
