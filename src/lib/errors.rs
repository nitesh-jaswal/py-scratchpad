
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

    impl std::fmt::Display for ScratchpadError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ScratchpadError::IoError(err) => {
                    match err.kind() {
                        std::io::ErrorKind::NotFound => write!(f, "Could not find file specified"),
                        _ => err.fmt(f)

                    }
                }
                _ => self.fmt(f),
            }
        }
    }
