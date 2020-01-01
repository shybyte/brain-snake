// error1.rs
use std::error::Error;
use std::fmt;

pub type SimpleResult<T> = Result<T, SimpleError>;

#[derive(Debug)]
pub struct SimpleError {
    details: String,
}

impl SimpleError {
    pub fn new(msg: &str) -> SimpleError {
        SimpleError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for SimpleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SimpleError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<std::io::Error> for SimpleError {
    fn from(err: std::io::Error) -> Self {
        SimpleError::new(err.description())
    }
}

impl From<std::convert::Infallible> for SimpleError {
    fn from(err: std::convert::Infallible) -> Self {
        SimpleError::new(err.description())
    }
}
