use crate::commands::*;
use colored::*;

pub fn display_help() -> String {
    let command_list = ScratchpadCommands::get_command_list();
    let mut help_string: Vec<String> = vec![format!("{}", ScratchpadCommands::cli_description().bright_purple())];
    command_list
        .iter()
        .for_each(|command| {
            help_string.push(
                format!(
                    "{}:\t{}", 
                    command.to_string().blue().bold(), 
                    ScratchpadCommands::single_line_command_description(command).italic()
            ))
        });
    help_string.join("\n")
}