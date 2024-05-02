mod requests;

use requests::*;
use crate::authentication::Pool;
use crate::authentication::models::errors::*;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use uuid::Uuid;

#[post("/login")]
pub async fn login(pool: web::Data<Pool>, req: web::Json<LoginRequest>) -> impl Responder {
    match pool.login(&req.username, &req.password).await {
        AuthorizationResult::Ok(user) => HttpResponse::Ok().json(user),
        AuthorizationResult::NotFound => HttpResponse::NotFound().json("Not found"),
        AuthorizationResult::Other => HttpResponse::InternalServerError().json("Server error"),
    }
}

#[post("/registration")]
pub async fn registration(pool: web::Data<Pool>,req: web::Json<RegistrationRequest>) -> impl Responder {
    match pool.registration(&req.username, &req.password).await {
        RegistrationResult::Ok(user) => HttpResponse::Ok().json(user),
        RegistrationResult::WeakPassword(cause) => HttpResponse::BadRequest().json(cause),
        RegistrationResult::AlreadyInUse => HttpResponse::BadRequest().json("Username already in user"),
        RegistrationResult::Other => HttpResponse::BadGateway().json("Bad gateway"),
    }
}

#[get("/users")]
pub async fn fetch_users(pool: web::Data<Pool>) -> impl Responder {
    match pool.get_users().await {
        Some(user_list) => HttpResponse::Ok().json(user_list),
        None => HttpResponse::NotFound().json("Not found")
    }
}

#[get("/users/{id}")]
pub async fn fetch_user(pool: web::Data<Pool>, uuid: web::Path<Uuid>) -> impl Responder {
    match pool.get(uuid.to_owned()).await {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json("Not found")
    }
}

#[patch("/users/{id}")]
pub async fn change_user(
    pool: web::Data<Pool>, 
    uuid: web::Path<Uuid>,
    req: web::Json<ChangeRequest>
    ) -> impl Responder {
    match pool.change_field(uuid.to_owned(), &req.field, &req.new_value).await {
        ChangeResult::Ok(user) => HttpResponse::Ok().json(user),
        ChangeResult::NotFoundField => HttpResponse::BadRequest().json("Not found field"),
        ChangeResult::WeakPassword(cause) => HttpResponse::BadRequest().json(cause),
        ChangeResult::InvalidPrivilege => HttpResponse::BadRequest().json("Invalid privilege"),
        ChangeResult::AlreadyInUse => HttpResponse::BadRequest().json("Username already in use")
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(pool: web::Data<Pool>, uuid: web::Path<Uuid>) -> impl Responder {
    match pool.delete(uuid.to_owned()).await {
        DeleteResult::Ok(user) => HttpResponse::Ok().json(user),
        DeleteResult::NotFound => HttpResponse::NotFound().json("Not found")
    }
}
