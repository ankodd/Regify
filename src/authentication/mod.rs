pub mod models;
use crate::authentication::models::{User, NewUser, outcome::*, privilege::*, constants};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::Error;
use dotenvy::dotenv;
use uuid::Uuid;
use std::env;
use std::str::FromStr;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

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
        if passwd.len() < 8 {
            return RegistrationResult::WeakPassword;
        }
        let new_user = NewUser {
            username: login.to_string().to_lowercase(),
            password: passwd.to_string(),
            privilege: UserPrivilege::Free
        };

        match diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(&mut self.0.get().unwrap()) {
            Ok(user) => RegistrationResult::Ok(user),
            Err(_) => RegistrationResult::Other,
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

    pub async fn change_field(&self, uuid: Uuid, field: &str, new: &str) -> ChangedResult {
        match field {  
            constants::USERNAME => {
                let user = diesel::update(users.find(uuid))
                    .set(username.eq(new))
                    .get_result::<User>(&mut self.0.get().unwrap()).unwrap();
                ChangedResult::Ok(user)
            },
            constants::PASSWORD => {
                if new.len() < 8 {
                    return ChangedResult::WeakPassword;
                } else {
                    let user = diesel::update(users.find(uuid))
                        .set(password.eq(new))
                        .get_result::<User>(&mut self.0.get().unwrap()).unwrap();
                    ChangedResult::Ok(user)
                }
            },
            constants::PRIVILEGE => {
                match UserPrivilege::from_str(new) {
                    Ok(new_privilege) => {
                        let user = diesel::update(users.find(uuid))
                            .set(privilege.eq(new_privilege))
                            .get_result::<User>(&mut self.0.get().unwrap()).unwrap();
                        ChangedResult::Ok(user)
                    },
                    Err(_) => ChangedResult::InvalidPrivilege
                }
            },
            _ => ChangedResult::NotFoundField
        }
    }
    
    pub async fn delete(&self, uuid: Uuid) -> DeletedResult {
        match diesel::delete(users)
            .filter(id.eq(uuid))
            .get_result::<User>(&mut self.0.get().unwrap()) {
                Ok(user) => DeletedResult::Ok(user),
                Err(_) => DeletedResult::NotFound,
        }
    }
}
