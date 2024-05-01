// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "user_privilege"))]
    pub struct UserPrivilege;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::UserPrivilege;

    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        privilege -> UserPrivilege,
        created_at -> Timestamptz,
    }
}
