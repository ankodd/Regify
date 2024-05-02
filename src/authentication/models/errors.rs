use crate::authentication::models::User;

type Cause = String;
pub enum ChangeResult {
    Ok(User),
    NotFoundField,
    WeakPassword(Cause),
    InvalidPrivilege,
    AlreadyInUse
}

pub enum RegistrationResult {
    Ok(User),
    WeakPassword(Cause),
    Other,
    AlreadyInUse
}

pub enum AuthorizationResult {
    Ok(User),
    NotFound,
    Other,
}

pub enum DeleteResult {
    Ok(User),
    NotFound
}

pub enum PasswordResult<'a> {
    Ok(&'a str),
    TooShort,
    NoDigits,
    NoLowercaseLetters,
    NoUppercaseLetters
}

impl ToString for PasswordResult {
    fn to_string(&self) -> String {
        match self {
            PasswordResult::Ok(some) => some.to_string(),
            PasswordResult::NoDigits => String::from("password has no digits"),
            PasswordResult::NoLowercaseLetters => String::from("password has no lowercase letters"),
            PasswordResult::NoUppercaseLetters => String::from("password has no uppercase letters"),
            PasswordResult::TooShort => String::from("password too short")
        }
    }
}