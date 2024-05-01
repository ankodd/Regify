use crate::database::{outcome::*, Pool};
use actix_web::{post,web,HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct RegistrationRequest {
    username: String,
    password: String,
}

#[post("/login")]
pub async fn login(pool: web::Data<Pool>, req: web::Json<LoginRequest>) -> impl Responder {
    match pool.login(&req.username, &req.password).await {
        AuthorizatiohOutcome::Ok(id) => HttpResponse::Ok().json(id),
        AuthorizatiohOutcome::NotFound => HttpResponse::NotFound().json("Not found"),
        AuthorizatiohOutcome::Other => HttpResponse::InternalServerError().json("Server error"),
    }
}

#[post("/registration")]
pub async fn registration(pool: web::Data<Pool>,req: web::Json<RegistrationRequest>) -> impl Responder {
    match pool.registration(&req.username, &req.password).await {
        RegistartionOutcome::Ok(user) => HttpResponse::Ok().json(user),
        RegistartionOutcome::WeakPassword => HttpResponse::BadRequest().json("Weak password"),
        RegistartionOutcome::AlreadyInUse => HttpResponse::BadRequest().json("Username already in use"),
        RegistartionOutcome::Other => HttpResponse::BadGateway().json("Bad gateway"),
    }
}
