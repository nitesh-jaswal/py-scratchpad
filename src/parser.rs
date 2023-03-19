use std::path::PathBuf;
use std::collections::HashMap;
use crate::errors::CliParseError;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum ScratchpadCommands {
    Config,
    Help,
    New,
    Open,
    Run,
    Delete
}

#[derive(Debug)]
pub enum CommandOptionValues {
    Int(i32),
    UInt(u32),
    Str(String),
    FilePath(PathBuf),
    None
}

#[derive(Debug)]
struct ScratchpadOperation {
    command: ScratchpadCommands,
    options: Option<HashMap<String, CommandOptionValues>>
}

fn parse_str(cmd_str: &str) -> Result<ScratchpadOperation, CliParseError> {
    Err(CliParseError::UnknownCommand)
}

#[cfg(test)]
mod tests {
    use crate::{parser::ScratchpadCommands, errors::CliParseError};
    use super::parse_str;


    #[test]
    fn test_parsing_run_command() {
        let cmd_str = "";
        let parsed_output = parse_str(cmd_str);
        assert_eq!(parsed_output.is_ok(), true);
        assert_eq!(ScratchpadCommands::Config, parsed_output.unwrap().command);
    }
    
    #[test]
    fn test_parsing_unknown_command() {
        let cmd_str = "";
        let parsed_output = parse_str(cmd_str);
        assert_eq!(parsed_output.is_err(), true);
        let flag = match parsed_output.unwrap_err() {
            CliParseError::UnknownCommand => true,
            _ => false,
        };
        assert_eq!(flag, true);
        
    }
}