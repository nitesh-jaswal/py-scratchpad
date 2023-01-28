mod cli_parser;
mod errors;

use cli_parser::{CLICommands, parse_args};
use errors::ScratchpadError;

// Print cli menu for configuration. Define default behaviour when starting first time
// Write individual functionality for virtual env creation, temp_dir mgmt and editor
fn main () -> Result<(), ScratchpadError> {
    let cli_commands =  parse_args()?;
    println!("Parsed command: {:?}", cli_commands);
    Ok(())
}