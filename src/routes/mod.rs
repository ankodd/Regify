mod requests;
mod error_message;

use error_message::JsonError;
use requests::*;
use crate::authentication::Pool;
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use uuid::Uuid;

#[post("/login")]
pub async fn login(pool: web::Data<Pool>, req: web::Json<LoginRequest>) -> impl Responder {
    use crate::authentication::models::errors::AuthorizationResult::*;
    match pool.login(&req.username, &req.password).await {
        Ok(user) => HttpResponse::Ok().json(user),
        NotFound => HttpResponse::NotFound().json(JsonError::new("not found")),
        Other => HttpResponse::InternalServerError().json(JsonError::new("server error"))
    }
}

#[post("/registration")]
pub async fn registration(pool: web::Data<Pool>,req: web::Json<RegistrationRequest>) -> impl Responder {
    use crate::authentication::models::errors::RegistrationResult::*;
    match pool.registration(&req.username, &req.password).await {
        Ok(user) => HttpResponse::Ok().json(user),
        WeakPassword(cause) => HttpResponse::BadRequest().json(JsonError::new(&cause)),
        AlreadyInUse => HttpResponse::BadRequest().json(JsonError::new("username already in use")),
        Other => HttpResponse::InternalServerError().json(JsonError::new("server error"))
    }
}

#[get("/users")]
pub async fn fetch_users(pool: web::Data<Pool>) -> impl Responder {
    match pool.get_users().await {
        Some(user_list) => HttpResponse::Ok().json(user_list),
        None => HttpResponse::NotFound().json(JsonError::new("not found")),
    }
}

#[get("/users/{id}")]
pub async fn fetch_user(pool: web::Data<Pool>, uuid: web::Path<Uuid>) -> impl Responder {
    match pool.get(uuid.to_owned()).await {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json(JsonError::new("not found")),
    }
}

#[patch("/users/{id}")]
pub async fn change_user(
    pool: web::Data<Pool>, 
    uuid: web::Path<Uuid>,
    req: web::Json<ChangeRequest>
    ) -> impl Responder {
    use crate::authentication::models::errors::ChangeResult::*;
    match pool.change_field(uuid.to_owned(), &req.field, &req.new_value).await {
        Ok(user) => HttpResponse::Ok().json(user),
        NotFoundField => HttpResponse::BadRequest().json(JsonError::new("not found field")),
        WeakPassword(cause) => HttpResponse::BadRequest().json(JsonError::new(&cause)),
        InvalidPrivilege => HttpResponse::BadRequest().json(JsonError::new("invalid privilege")),
        AlreadyInUse => HttpResponse::BadRequest().json(JsonError::new("username already in use"))
    }
}

#[delete("/users/{id}")]
pub async fn delete_user(pool: web::Data<Pool>, uuid: web::Path<Uuid>) -> impl Responder {
    use crate::authentication::models::errors::DeleteResult::*;
    match pool.delete(uuid.to_owned()).await {
        Ok(user) => HttpResponse::Ok().json(user),
        NotFound => HttpResponse::NotFound().json(JsonError::new("not found"))
    }
}
