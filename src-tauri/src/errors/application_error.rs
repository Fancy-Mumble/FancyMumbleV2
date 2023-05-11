use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ApplicationError {
    details: String,
}

impl ApplicationError {
    pub fn new(msg: &str) -> ApplicationError {
        ApplicationError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ApplicationError {
    fn description(&self) -> &str {
        &self.details
    }
}
