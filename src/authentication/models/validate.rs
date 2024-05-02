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

pub async fn is_validate_password(passwd: &str) -> PasswordResult {
    if passwd.len() < 8 {
        return PasswordResult::TooShort;
    }

    if !passwd.chars().any(|c| c.is_ascii_uppercase()) {
        return PasswordResult::NoUppercaseLetters;
    }

    if !passwd.chars().any(|c| c.is_ascii_lowercase()) {
        return PasswordResult::NoUppercaseLetters;
    }

    if !passwd.chars().any(|c| c.is_ascii_digit()) {
        return PasswordResult::NoDigits;
    }

    PasswordResult::Ok(String::from(passwd))
}
