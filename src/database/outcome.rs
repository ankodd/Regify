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
    Ok(Uuid),
    NotFound,
    Other,
}
