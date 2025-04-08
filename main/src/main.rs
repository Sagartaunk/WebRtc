use auth::{middlewarea , auth::{login , register ,  key_gen}};
use actix_web::{App, HttpServer, middleware, web};
use env_logger;
use log;
use local_ip_address::local_ip;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    key_gen();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let device_ip = local_ip().unwrap();
    let bind_address = format!("{}:80", device_ip);
    log::info!("Starting at http://{}", bind_address);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .wrap(middlewarea::Middleware)
            )
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .service(web::resource("/index.html").to(|| async { "Works i Guess lol" })) // Remove when testing done
    }).bind(bind_address)?
    .run()
    .await
}