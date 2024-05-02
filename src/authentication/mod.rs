pub mod models;
use crate::authentication::models::{User, NewUser, errors::*, privilege::*, constants, validate::*};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use dotenvy::dotenv;
use uuid::Uuid;
use std::env;
use std::future::Future;
use std::str::FromStr;
use crate::authentication::models::errors::ChangeResult::WeakPassword;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Clone)]
pub struct Pool(pub DbPool);

impl Pool {
    pub async fn init() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool");

        Self(pool)
    }

    pub async fn registration(&self, login: &str, passwd: &str) -> RegistrationResult {
        if !is_validate_username(&self.0,login).await {
            return RegistrationResult::AlreadyInUse;
        }
        match is_validate_password(passwd).await {
            PasswordResult::Ok(password) => {
                let new_user = NewUser {
                    username: login.to_string().to_lowercase(),
                    password,
                    privilege: UserPrivilege::Free
                };

                match diesel::insert_into(users)
                    .values(new_user)
                    .get_result::<User>(&mut self.0.get().unwrap()) {
                    Ok(user) => RegistrationResult::Ok(user),
                    Err(_) => RegistrationResult::Other,
                }
            },
            PasswordResult::TooShort => RegistrationResult::WeakPassword(PasswordResult::TooShort.to_string()),
            PasswordResult::NoUppercaseLetters => RegistrationResult::WeakPassword(PasswordResult::NoUppercaseLetters.to_string()),
            PasswordResult::NoLowercaseLetters => RegistrationResult::WeakPassword(PasswordResult::NoLowercaseLetters.to_string()),
            PasswordResult::NoDigits => RegistrationResult::WeakPassword(PasswordResult::NoDigits.to_string()),
        }
    }

    pub async fn login(&self, login: &str, passwd: &str) -> AuthorizationResult {
        match users
            .filter(username.eq(login.to_lowercase()))
            .get_result::<User>(&mut self.0.get().unwrap()) {
            Ok(user) => {
                if user.password == passwd {
                    AuthorizationResult::Ok(user)
                } else {
                    AuthorizationResult::NotFound
                }
            }
            Err(Error::NotFound) => AuthorizationResult::NotFound,
            _ => AuthorizationResult::Other,
        }
    }

    pub async fn get_users(&self) -> Option<Vec<User>> {
        match users.load::<User>(&mut self.0.get().unwrap()) {
            Ok(loaded) => Some(loaded),
            Err(_) => None 
        }
    }

    pub async fn get(&self, uuid: Uuid) -> Option<User> {
        match users.find(uuid).get_result::<User>(&mut self.0.get().unwrap()) {
            Ok(loaded) => Some(loaded),
            Err(_) => None
        }
    }

    pub async fn change_field(&self, uuid: Uuid, field: &str, new: &str) -> ChangeResult {
        match field {  
            constants::USERNAME => {
                if !self.is_good_username(new).await {
                    return ChangeResult::AlreadyInUse;
                }

                let user = diesel::update(users.find(uuid))
                    .set(username.eq(new))
                    .get_result::<User>(&mut self.0.get().unwrap()).unwrap();
                ChangeResult::Ok(user)
            },
            constants::PASSWORD => {
                match is_validate_password(new).await {
                    PasswordResult::Ok(new) => {
                        let user = diesel::update(users.find(uuid))
                            .set(password.eq(new))
                            .get_result::<User>(&mut self.0.get().unwrap()).unwrap();
                        ChangeResult::Ok(user)
                    }
                    PasswordResult::TooShort => WeakPassword(PasswordResult::TooShort.to_string()),
                    PasswordResult::NoUppercaseLetters => WeakPassword(PasswordResult::NoUppercaseLetters.to_string()),
                    PasswordResult::NoLowercaseLetters => WeakPassword(PasswordResult::NoLowercaseLetters.to_string()),
                    PasswordResult::NoDigits => WeakPassword(PasswordResult::NoDigits.to_string()),
                }
            },
            constants::PRIVILEGE => {
                match UserPrivilege::from_str(new) {
                    Ok(new_privilege) => {
                        let user = diesel::update(users.find(uuid))
                            .set(privilege.eq(new_privilege))
                            .get_result::<User>(&mut self.0.get().unwrap()).unwrap();
                        ChangeResult::Ok(user)
                    },
                    Err(_) => ChangeResult::InvalidPrivilege
                }
            },
            _ => ChangeResult::NotFoundField
        }
    }
    
    pub async fn delete(&self, uuid: Uuid) -> DeleteResult {
        match diesel::delete(users)
            .filter(id.eq(uuid))
            .get_result::<User>(&mut self.0.get().unwrap()) {
                Ok(user) => DeleteResult::Ok(user),
                Err(_) => DeleteResult::NotFound,
        }
    }
}
