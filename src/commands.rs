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

impl ScratchpadCommands {
    pub fn cli_description() -> String {
        "This is a CLI utility that easily allows you to CRUD python scratchpad files. A scratchpad file is sandbox enviornment for python experimentation\nUsage: `pyscratch COMMAND [OPTIONS]`
        ".to_string()
    }

    pub fn single_line_command_description(command: &ScratchpadCommands) -> String {
        match command {
            &Self::Config => "Allows you to configure the cli utiliy. If no arg provided it lists the config. `--new` will overwrite existing config. `--file` will accept .json config file path".to_string(),
            &Self::Help => "Displays help".to_string(),
            &Self::List => "Lists the scratchpad files created".to_string(),
            &Self::New => "Creates as new scratchpad file".to_string(),
            &Self::Open => "Opens a scratchpad file. If no `--id` provided, it opens the last modified file.".to_string(),
            &Self::Run => "Runs a scratchpad file. If no `--id` provided, it runs the last modified file.".to_string(),
            &Self::Delete => "Deletes a scratchpad file. If no `--id` provided, it runs the last modified file.".to_string(),
        }
    }
    
    pub fn get_command_list() -> [Self; 7] {
        use ScratchpadCommands::*;
        let command_list = [Config, Help, List, New, Open, Run, Delete];
        let flag = command_list.iter().all(|cmd| {
            match cmd {
                &Config => true,
                &Help => true,
                &List => true,
                &New => true,
                &Open => true,
                &Run => true,
                &Delete => true,
                _ => false
            }
        });
        if flag == false {
            panic!("Core CLI Error!! Not all commands have been implemented!")
        }
        command_list
    }

    pub fn valid_options(&self) -> Vec<&'static str> {
        todo!();
        // match self {
        //     &Self::Config => vec!["--new", "--file"],
        //     &Self::Help => vec![],
        //     &Self::List => vec![],
        //     &Self::New => vec![],
        //     &Self::Open => vec!["--id"],
        //     &Self::Run => vec!["--id"],
        //     &Self::Delete => vec!["--id"],
        // }
    }
}

impl FromStr for ScratchpadCommands {
    type Err = CliParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "config" => Ok(Self::Config),
            "help" => Ok(Self::Help),
            "list" => Ok(Self::List),
            "new" => Ok(Self::New),
            "open" => Ok(Self::Open),
            "run" => Ok(Self::Run),
            "delete" => Ok(Self::Delete),
            _ => Err(CliParseError::UnknownCommand)
        }
    }
}

impl ToString for ScratchpadCommands {
    fn to_string(&self) -> String {
        match &self {
            ScratchpadCommands::Config => "config".to_string(),
            ScratchpadCommands::Help => "help".to_string(),
            ScratchpadCommands::List => "list".to_string(),
            ScratchpadCommands::New => "new".to_string(),
            ScratchpadCommands::Open => "open".to_string(),
            ScratchpadCommands::Run => "run".to_string(),
            ScratchpadCommands::Delete => "delete".to_string(),
        }
    }
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
pub struct ScratchpadOperation {
    pub command: ScratchpadCommands,
    pub options: Option<HashMap<String, CommandOptionValues>>
}

