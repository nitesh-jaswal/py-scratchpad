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
#[derive(PartialEq)]
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
    let cmd_vec: Vec<&str> = cmd_str.split_ascii_whitespace().collect();
    match cmd_vec[0] {
        "help" => {
            return Ok(ScratchpadOperation { command: ScratchpadCommands::Help, options: None });
        }
        _ => {
            return Err(CliParseError::UnknownCommand)
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use crate::{parser::{ScratchpadCommands, ScratchpadOperation}, errors::CliParseError};
    use super::parse_str;

    #[rstest]
    #[case("help", Ok(ScratchpadOperation {command: ScratchpadCommands::Help, options: None}))]
    #[case("pingala_better_than_fibo", Err(CliParseError::UnknownCommand))]
    fn test_parsing_run_command(#[case] cmd_str: &str, #[case] expected_output: Result<ScratchpadOperation, CliParseError>) {
        let parsed_output = parse_str(cmd_str);
        match expected_output.as_ref() {
            Ok(output) => {
                assert_eq!(true, parsed_output.is_ok());
                let parsed_output = parsed_output.as_ref().unwrap();
                assert_eq!(&output.command, &parsed_output.command);
                match &output.options {
                    None => assert_eq!(parsed_output.options.is_none(), true),
                    Some(opts) => assert_eq!(parsed_output.options.as_ref().unwrap(), opts)
                }
            },
            Err(error) => {
                assert_eq!(true, parsed_output.is_err());
                let parsed_output = parsed_output.unwrap_err();
                assert_eq!(error, &parsed_output)
            }
        }
    }
}