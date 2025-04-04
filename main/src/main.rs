use auth::auth::{login , register ,  key_gen};
use actix_web::{App, HttpRequest, HttpServer, middleware, web};
use env_logger;
use log;




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    key_gen();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("starting HTTP server at http://localhost:8080");
    
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(login)
            .service(register)
            .service(web::resource("/index.html").to(|| async { "Working?" })) // Remove when testing done
    }).bind("192.168.1.10:8080")?
    .run()
    .await
}