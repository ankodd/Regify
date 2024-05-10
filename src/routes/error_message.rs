use serde::Serialize;
#[derive(Serialize)]
pub struct JsonError<'a> {
    message: &'a str
}

impl<'a> JsonError<'a> {
    pub fn new(msg: &'a str) -> Self {
        Self {
            message: msg
        }
    }
}