use lexopt::{Parser, Arg::*};
use std::{path::{Path, PathBuf}, os};
use std::ffi::OsString;

#[derive(Debug)]
enum CLICommands {
    Help,
    Config {
        editor: PathBuf,
        workspace: PathBuf,
        python_path: PathBuf,
        active_venv: PathBuf,
        last_created_file: Option<PathBuf>
    },
    New,
    List,
    Run {
        partial_id: Option<String>
    },
    Delete {
        partial_id: String
    },
    Open {
        partial_id: Option<String>
    }
}

fn parse_args() -> Result<CLICommands, lexopt::Error> {
    let mut parser = lexopt::Parser::from_env();
    let cli_arg = parser.next()?;
    if let Some(some_cmd) = cli_arg {
        let cmd = match some_cmd {
            Value(val) => { 
                match val.to_str() {
                    Some("help") => { CLICommands::Help },
                    Some("new") => { CLICommands::New },
                    Some("list") => { CLICommands::List },
                    Some("run") => { 
                        match parser.value() {
                            Ok(option) => {
                                if let Some(id) = option.to_str() {
                                    CLICommands::Run { partial_id: Some(String::from(id)) }
                                }
                                else {
                                    return Err(lexopt::Error::NonUnicodeValue(option))
                                }
                            },
                            Err(lexopt::Error::MissingValue { option: _ }) => { CLICommands::Run { partial_id: None } },
                            Err(err) => { return Err(err) } 
                        }
                    },
                    Some("delete") => { 
                        match parser.value() {
                            Ok(option) => {
                                if let Some(id) = option.to_str() {
                                    CLICommands::Delete{ partial_id: String::from(id) }
                                }
                                else {
                                    return Err(lexopt::Error::NonUnicodeValue(option))
                                }
                            },
                            Err(err) => { return Err(err) } 
                        }
                    },
                    Some("open") => { 
                        match parser.value() {
                            Ok(option) => {
                                if let Some(id) = option.to_str() {
                                    CLICommands::Open { partial_id: Some(String::from(id)) }
                                }
                                else {
                                    return Err(lexopt::Error::NonUnicodeValue(option))
                                }
                            },
                            Err(lexopt::Error::MissingValue { option: _ }) => { CLICommands::Open { partial_id: None } },
                            Err(err) => { return Err(err) } 
                        }
                    },
                    Some("config") => {
                        println!("ToDo. Menu will be walked through here");
                        CLICommands::Help
                    },
                    Some(_) | None => {
                        println!("Invalid usage! Please refer below for correct usage");
                        CLICommands::Help
                    }
                }
            },
            _ => {
                println!("Invalid usage! Please refer below for correct usage");
                CLICommands::Help
            }
        };
        return Ok(cmd)
    }
    else {
        println!("Invalid usage! Please refer below for correct usage");
        return Ok(CLICommands::Help)
    }
}

// Print cli menu for configuration. Define default behaviour when starting first time
// Write individual functionality for virtual env creation, temp_dir mgmt and editor
// Write 
fn main () -> Result<(), lexopt::Error> {
    let cli_commands =  parse_args()?;
    println!("Parsed command: {:?}", cli_commands);
    Ok(())
}