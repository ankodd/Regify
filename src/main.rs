use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod authentication;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = authentication::Pool::init().await;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(init_routes)
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(routes::login)
            .service(routes::registration),
    );
}
