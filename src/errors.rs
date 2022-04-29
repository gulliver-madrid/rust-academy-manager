use std::error::Error;
use std::fmt;

pub type SimpleResult = Result<(), SimpleError>;

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

/// If receive multiple args, first should be a format string
/// that will be used to format the other args
/// If receive an only arg, it should be a String
#[macro_export]
macro_rules! simple_error {
    ($e:expr) => {
        Err(SimpleError::new($e))
    };
    ($($e:expr),+) => {
        simple_error!(&format!(
            $($e),+
        ))
    };
}
