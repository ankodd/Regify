use actix_web::{web, App, HttpServer};
use env_logger::Env;
use actix_cors::Cors;

mod authentication;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = authentication::Pool::init().await;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(init_routes)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(cors)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(routes::login)
            .service(routes::registration)
            .service(routes::fetch_users)
            .service(routes::fetch_user)
            .service(routes::change_user)
            .service(routes::delete_user)
    );
}
