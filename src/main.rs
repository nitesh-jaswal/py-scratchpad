use lexopt::{Parser, Arg::*};
use std::{path::{Path, PathBuf}, os};
use std::ffi::OsString;
use colored::*;
use std::io::Result as IoResult;
use std::env;
use std::io;
use std::process::Command;
use std::convert::From;

#[derive(Debug)]
enum ScratchpadError {
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

fn display_interactive_cli_form() -> (String, String, String, Option<String>) {
    
    let mut input = String::new();
    let mut editor = "/usr/local/bin/code".to_string();
    println!("{}", "Note: If no value is provided, will default to vscode default install location".red());
    println!("Please enter path to your code editor[{}]:", editor.green());
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    input = input.trim().to_string();
    if !input.eq("") {
        editor = input;
    }
    
    let mut input = String::new();
    let mut workspace = format!("{}/.scratchpad/", env::var("HOME").unwrap());
    println!("{}", "Note: This is the directory that the scratchpad utility will use to save and maintain its files.".red());
    println!("Please enter path to the source directory[{}]:", workspace.green());
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    input = input.trim().to_string();
    if !input.eq("") {
        workspace = input;
    }

    let mut input = String::new();
    let mut python_path = String::new();
    println!("{}", "Note: This is the python version that will be used to create the scratchpad files.".red());
    println!("Please enter path to the python version[]:");
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    python_path = input.trim().to_string();
    

    let mut input = String::new();
    let mut active_venv = String::new();
    println!("{}", "Note: This is the virtual env that will be used to create and manage dendencies. If no value is provided a new one will be created.".red());
    println!("Please enter path to the source directory[]:");
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    active_venv = input.trim().to_string();
    match active_venv.as_str() {
        "" => (editor, workspace, python_path, None),
        _ => (editor, workspace, python_path, Some(active_venv)),
    }
}

fn parse_args() -> Result<CLICommands, ScratchpadError> {
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
                                    return Err(ScratchpadError::CLIError(lexopt::Error::NonUnicodeValue(option)))
                                }
                            },
                            Err(lexopt::Error::MissingValue { option: _ }) => { CLICommands::Run { partial_id: None } },
                            Err(err) => { return Err(ScratchpadError::CLIError(err)) } 
                        }
                    },
                    Some("delete") => { 
                        match parser.value() {
                            Ok(option) => {
                                if let Some(id) = option.to_str() {
                                    CLICommands::Delete{ partial_id: String::from(id) }
                                }
                                else {
                                    return Err(ScratchpadError::CLIError(lexopt::Error::NonUnicodeValue(option)))
                                }
                            },
                            Err(err) => { return Err(ScratchpadError::CLIError(err)) } 
                        }
                    },
                    Some("open") => { 
                        match parser.value() {
                            Ok(option) => {
                                if let Some(id) = option.to_str() {
                                    CLICommands::Open { partial_id: Some(String::from(id)) }
                                }
                                else {
                                    return Err(ScratchpadError::CLIError(lexopt::Error::NonUnicodeValue(option)))
                                }
                            },
                            Err(lexopt::Error::MissingValue { option: _ }) => { CLICommands::Open { partial_id: None } },
                            Err(err) => { return Err(ScratchpadError::CLIError(err)) } 
                        }
                    },
                    Some("config") => {
                        let (editor, workspace, python_path, active_venv) = display_interactive_cli_form();
                        let editor: PathBuf = PathBuf::from(editor);
                        let workspace: PathBuf = PathBuf::from(workspace);
                        let python_path: PathBuf = PathBuf::from(python_path);
                        
                        if !workspace.is_dir() {
                            println!("Workspace path directory does not exist. Creating...");
                            std::fs::create_dir_all(&workspace).expect(format!("Unable to create workspace directory in {}", workspace.as_path().to_str().unwrap()).as_str());
                        }
                        
                        if !editor.is_file() {
                            let err = std::io::Error::new(std::io::ErrorKind::NotFound, "Invalid editor path provided. Please verify input in config file.");
                            return Err(ScratchpadError::IoError(err))
                        }
                        if !python_path.is_file() {
                            let err = std::io::Error::new(std::io::ErrorKind::NotFound, "Invalid python path provided. Please verify input on config file.");
                            return Err(ScratchpadError::IoError(err))
                        }
                        let mut venv_dir = PathBuf::new();
                        if let Some(venv) = active_venv {
                            venv_dir.push(venv);
                            if !venv_dir.is_dir() && !venv_dir.join("bin").is_dir() {
                                let err = std::io::Error::new(std::io::ErrorKind::NotFound, "Invalid python venv path provided. Please verify input on config file.");
                                return Err(ScratchpadError::IoError(err))
                            }
                        }
                        else {
                            venv_dir.push("envs/DEFAULT_ENV");
                            venv_dir.push(workspace.as_path());
                            std::fs::create_dir_all(&venv_dir).expect(format!("Unable to create default env directory in {}", venv_dir.as_path().to_str().unwrap()).as_str());
                            Command::new(&python_path)
                            .args(["-m", "venv", venv_dir.as_path().to_str().unwrap()])
                            .output()
                            .expect("Unable to create default virtual env. Please check config input.");
                        }
                        CLICommands::Config { 
                            editor: editor, 
                            workspace: workspace, 
                            python_path: python_path, 
                            active_venv: venv_dir,
                            last_created_file: None 
                        }
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
fn main () -> Result<(), ScratchpadError> {
    let cli_commands =  parse_args()?;
    println!("Parsed command: {:?}", cli_commands);
    Ok(())
}