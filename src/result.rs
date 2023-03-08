use std::io::{Error, ErrorKind};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn err_invalid_data(message: &str) -> Box<dyn std::error::Error> {
    Box::new(Error::new(ErrorKind::InvalidData, message))
}
