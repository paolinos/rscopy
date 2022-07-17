use std::{fmt, error::Error};

#[derive(Debug)]
pub struct AppError {
    details: String
}

impl AppError {
    pub fn new(msg: &str) -> AppError {
        AppError{details: msg.to_string()}
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub type AppResult<T> = Result<T,AppError>;