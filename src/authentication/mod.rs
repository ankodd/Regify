pub mod models;
use crate::authentication::models::{User, NewUser, outcome::*, privilege::*};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::{DatabaseErrorKind, Error};
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

    pub async fn registration(&self, login: &str, passwd: &str) -> RegistartionOutcome {
        if passwd.len() < 8 {
            return RegistartionOutcome::WeakPassword;
        }
        let new_user = NewUser {
            username: login.to_string(),
            password: passwd.to_string(),
            privilege: UserPrivilege::Free
        };

        match diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(&mut self.0.get().unwrap()) {
            Ok(user) => RegistartionOutcome::Ok(user),
            Err(Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => RegistartionOutcome::AlreadyInUse,
            _ => RegistartionOutcome::Other,
        }
    }

    pub async fn login(&self, login: &str, passwd: &str) -> AuthorizatiohOutcome {
        match users
            .filter(username.eq(login.to_lowercase()))
            .get_result::<User>(&mut self.0.get().unwrap()) {
            Ok(user) => {
                if user.password == passwd {
                    AuthorizatiohOutcome::Ok(user)
                } else {
                    AuthorizatiohOutcome::NotFound
                }
            }
            Err(Error::NotFound) => AuthorizatiohOutcome::NotFound,
            _ => AuthorizatiohOutcome::Other,
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

    pub async fn change_field(&self, uuid: Uuid, field: &str, new: &str) -> ChangedOutcome {
        match field {  
            "username" => {
                let user = diesel::update(users.find(uuid))
                    .set(username.eq(new))
                    .get_result::<User>(&mut self.0.get().unwrap()).unwrap();
                ChangedOutcome::Ok(user)
            },
            "password" => {
                if new.len() < 8 { 
                    return ChangedOutcome::WeakPassword;
                } else {
                    let user = diesel::update(users.find(uuid))
                        .set(password.eq(new))
                        .get_result::<User>(&mut self.0.get().unwrap()).unwrap();
                    ChangedOutcome::Ok(user)
                }
            },
            "privilege" => {
                match UserPrivilege::from_str(new) {
                    Ok(new_privilege) => {
                        let user = diesel::update(users.find(uuid))
                            .set(privilege.eq(new_privilege))
                            .get_result::<User>(&mut self.0.get().unwrap()).unwrap();
                        ChangedOutcome::Ok(user)
                    },
                    Err(_) => ChangedOutcome::InvalidPrivilege
                }
            },
            _ => ChangedOutcome::NotFoundField
        }
    }
    
    pub async fn delete(&self, uuid: Uuid) -> DeletedOutcome {
        match diesel::delete(users)
            .filter(id.eq(uuid))
            .get_result::<User>(&mut self.0.get().unwrap()) {
                Ok(user) => DeletedOutcome::Ok(user),
                Err(_) => DeletedOutcome::NotFound,
        }
    }
}
