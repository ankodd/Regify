use crate::authentication::models::User;

pub enum ChangedResult {
    Ok(User),
    NotFoundField,
    WeakPassword,
    InvalidPrivilege,
}

pub enum RegistrationResult {
    Ok(User),
    WeakPassword,
    Other,
}

pub enum AuthorizationResult {
    Ok(User),
    NotFound,
    Other,
}

pub enum DeletedResult {
    Ok(User),
    NotFound
}
