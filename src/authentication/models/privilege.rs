use serde::Serialize;
use std::str::FromStr;

#[derive(diesel_derive_enum::DbEnum, Debug, Serialize)]
#[ExistingTypePath = "crate::schema::sql_types::UserPrivilege"]
pub enum UserPrivilege {
    Free,
    Super,
    Vip
}

impl ToString for UserPrivilege {
    fn to_string(&self) -> String {
        match self {
            UserPrivilege::Free => String::from("free"),
            UserPrivilege::Super => String::from("super"),
            UserPrivilege::Vip => String::from("vip")
        }
    }
}

impl FromStr for UserPrivilege {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "free" => Ok(UserPrivilege::Free),
            "super" => Ok(UserPrivilege::Super),
            "vip" => Ok(UserPrivilege::Vip),
            _ => Err("Invalid privilege")
        }
    }
}
