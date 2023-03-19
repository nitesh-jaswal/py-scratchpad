
use std::fmt;
use std::error::Error;
use std::convert::From;
    
#[derive(Debug)]
#[derive(PartialEq)]
pub enum CliParseError {
    MissingArgument(String),
    InvalidArgument(String),
    UnknownCommand,
}

impl fmt::Display for CliParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CliParseError::MissingArgument(val) => write!(f, "Missing argument: {}", val),
            CliParseError::InvalidArgument(val) => write!(f, "Invalid argument: {}", val),
            CliParseError::UnknownCommand => write!(f, "Unknown command"),
        }
    }
}

impl From<std::io::Error> for CliParseError {
    fn from(error: std::io::Error) -> Self {
        CliParseError::InvalidArgument(error.to_string())
    }
}

impl Error for CliParseError {}
    