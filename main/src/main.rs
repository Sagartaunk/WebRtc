use auth::auth::{login , register};
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(login)
            .service(register)
    }).bind("127.0.0.1:8080")?
    .run()
    .await
}