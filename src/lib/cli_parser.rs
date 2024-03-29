
// use crate::lib::ScratchpadError;

// use lexopt::Arg::*;
// use std::path::PathBuf;
// use colored::*;
// use std::env;
// use std::io;
// use std::process::Command;
// // use std::collections::HashMap;

// trait CLICommand {
//     fn get_help_string(&self) -> String;
//     fn parse_args<T>(args: Option<T>) -> Result<Self, lexopt::Error>
//     where T: std::iter::IntoIterator;
// }
// // Add arg to provide venv lcoation or auto detect it from current shell for every command
// #[derive(Debug)]
// pub enum ScratchpadCommand<T> 
// where T: CLICommand {
//     Config(ConfigArgs),
//     Run(RunArgs),
//     Delete(DeleteArgs),
//     Open(OpenArgs),
//     New(NewArgs),
//     Help(HelpArgs),
//     List(ListArgs),
// }
// struct HelpArgs;
// struct ListArgs;
// struct NewArgs;
// struct DeleteArgs {
//     partial_id: String
// }
// struct RunArgs {
//     partial_id: Option<String>
// }
// struct OpenArgs {
//     partial_id: Option<String>
// }
// struct ConfigArgs {
//     editor: PathBuf,
//     workspace: PathBuf,
//     python_path: PathBuf,
//     active_venv: PathBuf,
//     last_created_file: Option<PathBuf>
// }

// // Implementations
// impl CLICommand for HelpArgs {
//     fn get_help_string(&self) -> String {
//         "placeholder_text".to_string()
//     }

//     fn parse_args<T>(args: Option<T>) -> Result<Self, lexopt::Error>
//         where T: std::iter::IntoIterator {
//         Ok(HelpArgs)
//     }
// }
// impl CLICommand for ListArgs {
//     fn get_help_string(&self) -> String {
//         "placeholder_text".to_string()
//     }

//     fn parse_args<T>(args: Option<(String)>) -> Result<Self, lexopt::Error>
//         where T: std::iter::IntoIterator {
//         Ok(ListArgs)
//     }
// }
// impl CLICommand for NewArgs {
//     fn get_help_string(&self) -> String {
//         "placeholder_text".to_string()
//     }

//     fn parse_args<T>(args: Option<String>) -> Result<Self, lexopt::Error>
//         where T: std::iter::IntoIterator {
//         Ok(NewArgs)
//     }
// }

// impl CLICommand for DeleteArgs {
//     fn get_help_string(&self) -> String {
//         "placeholder_text".to_string()
//     }

//     fn parse_args<T>(args: Option<String>) -> Result<Self, lexopt::Error>
//         where T: std::iter::IntoIterator {
//         match args {
//             Some(val) => Ok(DeleteArgs { partial_id: val }),
//             None => Err(lexopt::Error::MissingValue { option: "File id was expected but not found" })
//         }
//     }
// }

// impl CLICommand for RunArgs {
//     fn get_help_string(&self) -> String {
//         "placeholder_text".to_string()
//     }

//     fn parse_args<T>(args: Option<String>) -> Result<Self, lexopt::Error>
//         where T: std::iter::IntoIterator {
//             Ok(RunArgs{ partial_id: args })
//     }
// }

// impl CLICommand for OpenArgs {
//     fn get_help_string(&self) -> String {
//         "placeholder_text".to_string()
//     }

//     fn parse_args<T>(args: Option<String>) -> Result<Self, lexopt::Error>
//         where T: std::iter::IntoIterator {
//             Ok(OpenArgs{ partial_id: args })
//     }
// }

// impl ConfigArgs {
//     fn display_interactive_cli_form() -> (String, String, String, Option<String>) {
    
//         let mut input = String::new();
//         let mut editor = "/usr/local/bin/code".to_string();
//         println!("{}", "Note: If no value is provided, will default to vscode default install location".red());
//         println!("Please enter path to your code editor[{}]:", editor.green());
//         io::stdin().read_line(&mut input)
//         .expect("Failed to read line");
//         input = input.trim().to_string();
//         if !input.eq("") {
//             editor = input;
//         }
        
//         let mut input = String::new();
//         let mut workspace = format!("{}/.scratchpad/", env::var("HOME").unwrap());
//         println!("{}", "Note: This is the directory that the scratchpad utility will use to save and maintain its files.".red());
//         println!("Please enter path to the source directory[{}]:", workspace.green());
//         io::stdin().read_line(&mut input)
//         .expect("Failed to read line");
//         input = input.trim().to_string();
//         if !input.eq("") {
//             workspace = input;
//         }
    
//         let mut input = String::new();
//         let mut python_path = String::new();
//         println!("{}", "Note: This is the python version that will be used to create the scratchpad files.".red());
//         println!("Please enter path to the python version[]:");
//         io::stdin().read_line(&mut input)
//         .expect("Failed to read line");
//         python_path = input.trim().to_string();
        
    
//         let mut input = String::new();
//         let mut active_venv = String::new();
//         println!("{}", "Note: This is the virtual env that will be used to create and manage dendencies. If no value is provided a new one will be created.".red());
//         println!("Please enter path to the source directory[]:");
//         io::stdin().read_line(&mut input)
//         .expect("Failed to read line");
//         active_venv = input.trim().to_string();
    
//         match active_venv.as_str() {
//             "" => return (editor, workspace, python_path, None),
//             _ => return (editor, workspace, python_path, Some(active_venv)),
//         }
//     }
// }

// impl CLICommand for ConfigArgs {
//     fn get_help_string(&self) -> String {
//         "Test".to_string()
//     }

//     // TODO: Move to HashMap?
//     fn parse_args(args: Option<(String, String, String, Option<String>)>) -> Result<Self, lexopt::Error>
//     {
//         Ok(ConfigArgs { 
//             editor: PathBuf::from("value"), 
//             workspace: PathBuf::from("value"), 
//             python_path: PathBuf::from("value"), 
//             active_venv: PathBuf::from("value"), 
//             last_created_file: None
//         })
//     }
// }

// pub fn parse_args() -> Result<ScratchpadCommand, ScratchpadError> {
//     let mut parser = lexopt::Parser::from_env();
//     let cli_arg = parser.next()?;
//     if let Some(some_cmd) = cli_arg {
//         let cmd = match some_cmd {
//             Value(val) => { 
//                 match val.to_str() {
//                     Some("help") => { ScratchpadCommand::Help },
//                     Some("new") => { ScratchpadCommand::New },
//                     Some("list") => { ScratchpadCommand::List },
//                     Some("run") => { 
//                         match parser.value() {
//                             Ok(option) => {
//                                 if let Some(id) = option.to_str() {
//                                     ScratchpadCommand::Run { partial_id: Some(String::from(id)) }
//                                 }
//                                 else {
//                                     return Err(ScratchpadError::CLIError(lexopt::Error::NonUnicodeValue(option)))
//                                 }
//                             },
//                             Err(lexopt::Error::MissingValue { option: _ }) => { ScratchpadCommand::Run { partial_id: None } },
//                             Err(err) => { return Err(ScratchpadError::CLIError(err)) } 
//                         }
//                     },
//                     Some("delete") => { 
//                         match parser.value() {
//                             Ok(option) => {
//                                 if let Some(id) = option.to_str() {
//                                     ScratchpadCommand::Delete{ partial_id: String::from(id) }
//                                 }
//                                 else {
//                                     return Err(ScratchpadError::CLIError(lexopt::Error::NonUnicodeValue(option)))
//                                 }
//                             },
//                             Err(err) => { return Err(ScratchpadError::CLIError(err)) } 
//                         }
//                     },
//                     Some("open") => { 
//                         match parser.value() {
//                             Ok(option) => {
//                                 if let Some(id) = option.to_str() {
//                                     ScratchpadCommand::Open { partial_id: Some(String::from(id)) }
//                                 }
//                                 else {
//                                     return Err(ScratchpadError::CLIError(lexopt::Error::NonUnicodeValue(option)))
//                                 }
//                             },
//                             Err(lexopt::Error::MissingValue { option: _ }) => { ScratchpadCommand::Open { partial_id: None } },
//                             Err(err) => { return Err(ScratchpadError::CLIError(err)) } 
//                         }
//                     },
//                     Some("config") => {
//                         let (editor, workspace, python_path, active_venv) = display_interactive_cli_form();
//                         let editor: PathBuf = PathBuf::from(editor);
//                         let workspace: PathBuf = PathBuf::from(workspace);
//                         let python_path: PathBuf = PathBuf::from(python_path);
                        
//                         if !workspace.is_dir() {
//                             println!("Workspace path directory does not exist. Creating...");
//                             std::fs::create_dir_all(&workspace).expect(&format!("Unable to create workspace directory in {}", workspace.display()));
//                         }
                        
//                         if !editor.is_file() {
//                             let err = std::io::Error::new(std::io::ErrorKind::NotFound, "Invalid editor path provided. Please verify input in config file.");
//                             return Err(ScratchpadError::IoError(err))
//                         }
//                         if !python_path.is_file() {
//                             let err = std::io::Error::new(std::io::ErrorKind::NotFound, "Invalid python path provided. Please verify input on config file.");
//                             return Err(ScratchpadError::IoError(err))
//                         }
//                         let mut venv_dir = PathBuf::new();
//                         if let Some(venv) = active_venv {
//                             venv_dir.push(venv);
//                             if !venv_dir.is_dir() && !venv_dir.join("bin").is_dir() {
//                                 let err = std::io::Error::new(std::io::ErrorKind::NotFound, "Invalid python venv path provided. Please verify input on config file.");
//                                 return Err(ScratchpadError::IoError(err))
//                             }
//                         }
//                         else {
//                             venv_dir.push(workspace.as_path());
//                             venv_dir.push("envs/DEFAULT_ENV/");
//                             println!("Creating a venv in {}", venv_dir.to_str().unwrap());
//                             std::fs::create_dir_all(&venv_dir).expect(&format!("Unable to create default env directory in {}", venv_dir.display()));
//                             Command::new(&python_path)
//                             .args(&["-m", "venv", &venv_dir.to_string_lossy()])
//                             .output()
//                             .expect("Unable to create default virtual env. Please check config input.");
//                         }
//                         ScratchpadCommand::Config { 
//                             editor: editor, 
//                             workspace: workspace, 
//                             python_path: python_path, 
//                             active_venv: venv_dir,
//                             last_created_file: None 
//                         }
//                     },
//                     Some(_) | None => {
//                         println!("Invalid usage! Please refer below for correct usage");
//                         ScratchpadCommand::Help
//                     }
//                 }
//             },
//             _ => {
//                 println!("Invalid usage! Please refer below for correct usage");
//                 ScratchpadCommand::Help
//             }
//         };
//         return Ok(cmd)
//     }
//     else {
//         println!("Invalid usage! Please refer below for correct usage");
//         return Ok(ScratchpadCommand::Help)
//     }
// }


