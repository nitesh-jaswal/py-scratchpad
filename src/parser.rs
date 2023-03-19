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
    List,
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
        "help" => return Ok(ScratchpadOperation { command: ScratchpadCommands::Help, options: None }),
        "list" => return Ok(ScratchpadOperation { command: ScratchpadCommands::List, options: None }),
        "new" => return Ok(ScratchpadOperation { command: ScratchpadCommands::New, options: None }),
        "delete" => {
            cursor = cursor + 1;
            match cmd_vec.get(cursor) {
                Some(&"--id") => {
                    cursor = cursor + 1;
                    if let Some(file_id) = cmd_vec.get(cursor) {
                        return Ok(ScratchpadOperation { command: ScratchpadCommands::Delete, options: Some(vec![("--id".to_string(), CommandOptionValues::Str(file_id.to_string()))].into_iter().collect::<HashMap<String, CommandOptionValues>>()) })
                    }
                    else {
                        return Err(CliParseError::MissingArgument("Please enter the complete or partial id of the file to be deleted".to_string()))
                    }
                },
                Some(&val) => return Err(CliParseError::InvalidArgument(val.to_string())),
                None => return Ok(ScratchpadOperation { command: ScratchpadCommands::Delete, options: None })
            }
        },  
        "open" => {
            cursor = cursor + 1;
            match cmd_vec.get(cursor) {
                Some(&"--id") => {
                    cursor = cursor + 1;
                    if let Some(file_id) = cmd_vec.get(cursor) {
                        return Ok(ScratchpadOperation { command: ScratchpadCommands::Open, options: Some(vec![("--id".to_string(), CommandOptionValues::Str(file_id.to_string()))].into_iter().collect::<HashMap<String, CommandOptionValues>>()) })
                    }
                    else {
                        return Err(CliParseError::MissingArgument("Please enter the complete or partial id of the file to be deleted".to_string()))
                    }
                },
                Some(&val) => return Err(CliParseError::InvalidArgument(val.to_string())),
                None => return Ok(ScratchpadOperation { command: ScratchpadCommands::Open, options: None })
            }
        },
        "run" => {
            cursor = cursor + 1;
            match cmd_vec.get(cursor) {
                Some(&"--id") => {
                    cursor = cursor + 1;
                    if let Some(file_id) = cmd_vec.get(cursor) {
                        return Ok(ScratchpadOperation { command: ScratchpadCommands::Run, options: Some(vec![("--id".to_string(), CommandOptionValues::Str(file_id.to_string()))].into_iter().collect::<HashMap<String, CommandOptionValues>>()) })
                    }
                    else {
                        return Err(CliParseError::MissingArgument("Please enter the complete or partial id of the file to be deleted".to_string()))
                    }
                },
                Some(&val) => return Err(CliParseError::InvalidArgument(val.to_string())),
                None => return Ok(ScratchpadOperation { command: ScratchpadCommands::Run, options: None })
            }
        },
        "config" => {
            cursor = cursor + 1;
            match cmd_vec.get(cursor) {
                Some(&"--new") => return Ok(ScratchpadOperation{ command: ScratchpadCommands::Config, options: Some(vec![("--new".to_string(), CommandOptionValues::None)].into_iter().collect::<HashMap<String, CommandOptionValues>>())}),
                Some(&"--file") => {
                    cursor = cursor + 1;
                    if let Some(&fpath) = cmd_vec.get(cursor) {
                        let fpath = PathBuf::from(fpath);
                        return Ok(ScratchpadOperation{ command: ScratchpadCommands::Config, options: Some(vec![("--file".to_string(), CommandOptionValues::FilePath(fpath))].into_iter().collect::<HashMap<String, CommandOptionValues>>())})
                    }
                    else {
                        return Err(CliParseError::MissingArgument("The path to .json config file is required but not provided".to_string()))
                    }
                },
                Some(val) => return Err(CliParseError::InvalidArgument(val.to_string())),
                None => return Ok(ScratchpadOperation{ command: ScratchpadCommands::Config, options: None})
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

    use crate::{parser::{ScratchpadCommands, ScratchpadOperation, CommandOptionValues}, errors::CliParseError};
    use super::parse_str;

    #[rstest]
    #[case::help("help", Ok(ScratchpadOperation {command: ScratchpadCommands::Help, options: None}))]
    #[case::list("list", Ok(ScratchpadOperation {command: ScratchpadCommands::List, options: None}))]
    #[case::new("new", Ok(ScratchpadOperation {command: ScratchpadCommands::New, options: None}))]
    #[case::delete_default("delete", Ok(ScratchpadOperation {command: ScratchpadCommands::Delete, options: None}))]
    #[case::delete_id_valid("delete --id 5oM3iD", Ok(ScratchpadOperation {command: ScratchpadCommands::Delete, options: Some(vec![("--id".to_string(), CommandOptionValues::Str("5oM3iD".to_string()))].into_iter().collect::<HashMap<String, CommandOptionValues>>())}))]
    #[case::delete_id_missing_value("delete --id", Err(CliParseError::MissingArgument("Please enter the complete or partial id of the file to be deleted".to_string())))]
    #[case::delete_invalid_option("delete --test_invalid", Err(CliParseError::InvalidArgument("--test_invalid".to_string())))]
    #[case::open_default("open", Ok(ScratchpadOperation {command: ScratchpadCommands::Open, options: None}))]
    #[case::open_id_valid("open --id 5oM3iD", Ok(ScratchpadOperation {command: ScratchpadCommands::Open, options: Some(vec![("--id".to_string(), CommandOptionValues::Str("5oM3iD".to_string()))].into_iter().collect::<HashMap<String, CommandOptionValues>>())}))]
    #[case::open_id_missing_value("open --id", Err(CliParseError::MissingArgument("Please enter the complete or partial id of the file to be deleted".to_string())))]
    #[case::open_invalid_option("open --test_invalid", Err(CliParseError::InvalidArgument("--test_invalid".to_string())))]
    #[case::run_default("run", Ok(ScratchpadOperation {command: ScratchpadCommands::Run, options: None}))]
    #[case::run_id_valid("run --id 5oM3iD", Ok(ScratchpadOperation {command: ScratchpadCommands::Run, options: Some(vec![("--id".to_string(), CommandOptionValues::Str("5oM3iD".to_string()))].into_iter().collect::<HashMap<String, CommandOptionValues>>())}))]
    #[case::run_id_missing_value("run --id", Err(CliParseError::MissingArgument("Please enter the complete or partial id of the file to be deleted".to_string())))]
    #[case::run_invalid_option("run --test_invalid", Err(CliParseError::InvalidArgument("--test_invalid".to_string())))]
    #[case::config_default("config", Ok(ScratchpadOperation {command: ScratchpadCommands::Config, options: None}))]
    #[case::config_new("config --new", Ok(ScratchpadOperation { command: ScratchpadCommands::Config, options: Some(vec![("--new".to_string(), CommandOptionValues::None)].into_iter().collect::<HashMap<String, CommandOptionValues>>()) }))]
    #[case::config_file_valid("config --file /path/to/some/file", Ok(ScratchpadOperation { command: ScratchpadCommands::Config, options: Some(vec![("--file".to_string(), CommandOptionValues::FilePath(PathBuf::from("/path/to/some/file")))].into_iter().collect::<HashMap<String, CommandOptionValues>>()) }))]
    #[case::config_file_mising_value("config --file", Err(CliParseError::MissingArgument("The path to .json config file is required but not provided".to_string())))]
    #[case::config_invalid_option("config --test_invalid", Err(CliParseError::InvalidArgument("--test_invalid".to_string())))]
    #[case::unknown_command("pingala_better_than_fibonacci", Err(CliParseError::UnknownCommand))]
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