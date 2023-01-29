mod cli_parser;
mod errors;

use cli_parser::{CLICommands, parse_args};
use errors::ScratchpadError;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::seq::SliceRandom;


fn get_unique_id() -> String {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error getting timestamp");
    let timestamp = timestamp.as_secs().to_string();
    let base_string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
    let id = base_string.choose_multiple(&mut rand::thread_rng(), 4).map(|x| -> u8 {*x}).collect::<Vec<_>>();
    format!("{}_{}", String::from_utf8_lossy(id.as_slice()).to_string(), timestamp)
}

fn display_help() {
    let description = "This is a cli utility that helps you easily create and manage temporary python files for quick tests \
    and experimentation";
    let usage = "scratchpad [COMMAND] [OPTIONS]";
    let available_commands = "Available commands:
delete  [None | <file_id>]          Allows you to delete one of the existing scratchpad python files created. You can specify the file 
                                    to delete by stating the partial or complete file-id. If no file id is provided it deletes the last created file.
run     [None | <file_id>]          Allows you to run one of the existing scratchpad python files created. You can specify the file to run by stating 
                                    the partial or complete file-id. If no file id is provided it runs the last created file
open    [<file_id>]                 Opens a the file for the file-id provided in the configured code editor
config                              This starts and interactive cli menu that helps you configure the scratchpad utility. Saves it as a config.json 
                                    file in the workspace directory, backing up the old file if it exists.
list                                Lists the existing scratchpad files along with creation date-time and a description if available
new                                 Creates a new scratchpad python file
help                                Displays the help";
    println!("{}\n{}\n\n{}", description, usage, available_commands);
}

fn process_new_config(config: CLICommands) -> Result<(), ScratchpadError> {
    match config {
        CLICommands::Config { editor, workspace, python_path, active_venv, last_created_file } => {
            // Check if a config.json already exists and display a message stating the same
            let config_json = workspace.with_file_name("config.json");
            if config_json.exists() {
                println!("An existing config file found. Backing up file and replacing existing config.json");
                // backup existing file
                let backup_config_filename = format!("config_{}.json", get_unique_id());
                let backup_config_json = config_json.with_file_name(backup_config_filename);
                std::fs::copy(&config_json, &backup_config_json)?;
            }
            let config_file = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(config_json)?;
            let last_created_file = match last_created_file {
                Some(fpath) => fpath.to_str().unwrap(),
                None => "null"
            }
            let json_content = format!("{{
\"editor\": \"{}\",
\"workspace\": \"{}\",
\"python_path\": \"{}\",
\"active_venv\": \"{}\",
\"last_created_file\": \"{}\",
}}", editor.to_str().unwrap(), workspace.to_str().unwrap(), python_path.to_str().unwrap(), active_venv.to_str().unwrap(), last_created_file);
            // Write to config.json
            config_file

        },
        _ => panic!("Internal error! Expected CLICommands::Config enum variant. Incorrect usage of function")
    };
    Ok(())
}
// Print cli menu for configuration. Define default behaviour when starting first time
// Write individual functionality for virtual env creation, temp_dir mgmt and editor
fn main () -> Result<(), ScratchpadError> {
    let cli_command =  parse_args()?;
    match cli_command {
        CLICommands::Config { .. } => { println!("New config received! {:?}", cli_command)},
        CLICommands::Delete { .. } => { println!("Delete request received! {:?}", cli_command)},
        CLICommands::Run { .. } => { println!("Run received! {:?}", cli_command)},
        CLICommands::Open { .. } => { println!("Open received! {:?}", cli_command)},
        CLICommands::List => { println!("List received! {:?}", cli_command)},
        CLICommands::New => { println!("New received! {:?}", cli_command)},
        CLICommands::Help => display_help(),
        _ => {}
    };
    Ok(())
}