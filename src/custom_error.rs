use std::error::Error;
use std::fmt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CustomError {
    details: String
}

impl CustomError {
    pub fn new(msg: &str) -> Self {
        Self{details: msg.to_string()}
    }
}


impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.details)
    }
}

impl Error for CustomError {
    fn description(&self) -> &str {
        &self.details
    }
}