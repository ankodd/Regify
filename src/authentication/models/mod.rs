pub mod errors;
pub mod privilege;
pub mod constants;
pub mod validate;

use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::authentication::models::privilege::*;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password: String,
    pub privilege: UserPrivilege,
    pub created_at: DateTime<Utc>
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub privilege: UserPrivilege,
}
