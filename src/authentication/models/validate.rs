use diesel::prelude::*;
use crate::authentication::DbPool;
use crate::authentication::models::User;
use crate::schema::users::dsl::*;
use crate::authentication::models::errors::PasswordResult;

pub async fn is_validate_username(pool: &DbPool, login: &str) -> bool {
    if users
        .filter(username.eq(login))
        .first::<User>(&mut pool.get().unwrap())
        .optional()
        .unwrap_or(None)
        .is_some() {
        true
    } else {
        false
    }
}

pub async fn is_validate_password(password: &str) -> PasswordResult {
    if password.len() < 8 {
        PasswordResult::TooShort
    }

    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        PasswordResult::NoUppercaseLetters
    }

    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        PasswordResult::NoUppercaseLetters
    }

    if !password.chars().any(|c| c.is_ascii_digit()) {
        PasswordResult::NoDigits
    }

    PasswordResult::Ok(password)
}