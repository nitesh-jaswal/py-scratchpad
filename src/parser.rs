use std::ops::Add;
use std::path::PathBuf;
use std::collections::HashMap;
use std::str::FromStr;
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
    let mut cursor: usize = 0;
    match cmd_vec[cursor] {
        "help" => {
            return Ok(ScratchpadOperation { command: ScratchpadCommands::Help, options: None });
        },
        "config" => {
            cursor = cursor + 1;
            println!("Cursor: {}", cursor);
            if let Some(&opt) = cmd_vec.get(cursor) {
                match opt {
                    "--new" => return Ok(ScratchpadOperation{ command: ScratchpadCommands::Config, options: Some(vec![("--new".to_string(), CommandOptionValues::None)].into_iter().collect::<HashMap<String, CommandOptionValues>>())}),
                    "--file" => {
                        cursor = cursor + 1;
                        if let Some(&fpath) = cmd_vec.get(cursor) {
                            let fpath = PathBuf::from(fpath);
                            return Ok(ScratchpadOperation{ command: ScratchpadCommands::Config, options: Some(vec![("--file".to_string(), CommandOptionValues::FilePath(fpath))].into_iter().collect::<HashMap<String, CommandOptionValues>>())})
                        }
                        else {
                            return Err(CliParseError::MissingArgument("The path to .json config file is required but not provided".to_string()))
                        }
                    },
                    _ => return Err(CliParseError::InvalidArgument(opt.to_string()))
                }
            }
            else {
                return Ok(ScratchpadOperation{ command: ScratchpadCommands::Config, options: None})
            }
        },
        _ => {
            return Err(CliParseError::UnknownCommand)
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::{collections::HashMap, path::PathBuf};
    use std::str::FromStr;

    use crate::{parser::{ScratchpadCommands, ScratchpadOperation, CommandOptionValues}, errors::CliParseError};
    use super::parse_str;

    #[rstest]
    #[case::help("help", Ok(ScratchpadOperation {command: ScratchpadCommands::Help, options: None}))]
    #[case::config_default("config", Ok(ScratchpadOperation {command: ScratchpadCommands::Config, options: None}))]
    #[case::config_new("config --new", Ok(ScratchpadOperation { command: ScratchpadCommands::Config, options: Some(vec![("--new".to_string(), CommandOptionValues::None)].into_iter().collect::<HashMap<String, CommandOptionValues>>()) }))]
    #[case::config_file_valid("config --file /path/to/some/file", Ok(ScratchpadOperation { command: ScratchpadCommands::Config, options: Some(vec![("--file".to_string(), CommandOptionValues::FilePath(PathBuf::from("/path/to/some/file")))].into_iter().collect::<HashMap<String, CommandOptionValues>>()) }))]
    #[case::config_file_mising_argument("config --file", Err(CliParseError::MissingArgument("The path to .json config file is required but not provided".to_string())))]
    #[case::unknown_command("pingala_better_than_fibo", Err(CliParseError::UnknownCommand))]
    fn test_parsing_str_to_command(#[case] cmd_str: &str, #[case] expected_output: Result<ScratchpadOperation, CliParseError>) {
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