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
            UserPrivilege::Free => String::from("Free"),
            UserPrivilege::Super => String::from("Super"),
            UserPrivilege::Vip => String::from("Vip")
        }
    }
}

impl FromStr for UserPrivilege {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Free" => Ok(UserPrivilege::Free),
            "Super" => Ok(UserPrivilege::Super),
            "Vip" => Ok(UserPrivilege::Vip),
            _ => Err("Invalid privilege")
        }
    }
}
