use crate::authentication::models::User;

pub enum ChangedOutcome {
    Ok(User),
    NotFoundField,
    WeakPassword,
    InvalidPrivilege,
}

pub enum RegistartionOutcome {
    Ok(User),
    AlreadyInUse,
    WeakPassword,
    Other,
}

pub enum AuthorizatiohOutcome {
    Ok(User),
    NotFound,
    Other,
}

pub enum DeletedOutcome {
    Ok(User),
    NotFound
}
