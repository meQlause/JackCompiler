use crate::prelude::*;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum ParsingError {
    UnexpectedToken(String),
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ParsingError::UnexpectedToken(token) => {
                write!(f, "Unexpected token: {token}")
            }
        }
    }
}
