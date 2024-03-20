use actix_web::{App, HttpServer};

mod db;
mod router;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(router::routes::config)
        // Add more routes for creating, updating, and deleting users
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
