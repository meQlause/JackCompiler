use thiserror::Error;
#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("Something went wrong!")]
    UnexpectedToken,
}