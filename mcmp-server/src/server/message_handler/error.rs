use std::fmt::Display;

use anyhow::Error;
pub struct HandlerError {
    pub msg: String,
    pub id: usize
}


impl HandlerError {
    pub fn new<S: Into<String>, I: Into<usize>>(s: S, id: I) -> Self {
        Self {
            msg: s.into(),
            id: id.into(),
        }
    }
}

impl From<Error> for HandlerError {
    fn from(value: Error) -> Self {
        Self {
            msg: value.to_string(),
            id: 0,
        }
    }
}



impl Display for HandlerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}) {}", self.id, self.msg)?;
        Ok(())
    }
}