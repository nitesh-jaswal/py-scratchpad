use std::convert::From;

#[derive(Debug)]
pub enum ScratchpadError {
    IoError(std::io::Error),
    CLIError(lexopt::Error)
}

impl From<std::io::Error> for ScratchpadError {
    fn from(error: std::io::Error) -> Self {
        ScratchpadError::IoError(error)
    }
}

impl From<lexopt::Error> for ScratchpadError {
    fn from(error: lexopt::Error) -> Self {
        ScratchpadError::CLIError(error)
    }
}