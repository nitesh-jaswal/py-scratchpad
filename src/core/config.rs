// use std::env;
// use std::io::{self, BufRead, Write};

// fn display_interactive_cli_form() -> (String, String, String, Option<String>) {
//     let mut editor = "/usr/local/bin/code".to_string();
//     println!("{}", "Note: If no value is provided, will default to vscode default install location".red());
//     println!("Please enter path to your code editor[{}]:", editor.green());
//     io::stdin().read_line(&mut input)
//     .expect("Failed to read line");
//     input = input.trim().to_string();
//     if !input.eq("") {
//         editor = input;
//     }
    
//     let mut input = String::new();
//     let mut workspace = format!("{}/.scratchpad/", env::var("HOME").unwrap());
//     println!("{}", "Note: This is the directory that the scratchpad utility will use to save and maintain its files.".red());
//     println!("Please enter path to the source directory[{}]:", workspace.green());
//     io::stdin().read_line(&mut input)
//     .expect("Failed to read line");
//     input = input.trim().to_string();
//     if !input.eq("") {
//         workspace = input;
//     }

//     let mut input = String::new();
//     let mut python_path = String::new();
//     println!("{}", "Note: This is the python version that will be used to create the scratchpad files.".red());
//     println!("Please enter path to the python version[]:");
//     io::stdin().read_line(&mut input)
//     .expect("Failed to read line");
//     python_path = input.trim().to_string();
    

//     let mut input = String::new();
//     let mut active_venv = String::new();
//     println!("{}", "Note: This is the virtual env that will be used to create and manage dendencies. If no value is provided a new one will be created.".red());
//     println!("Please enter path to the source directory[]:");
//     io::stdin().read_line(&mut input)
//     .expect("Failed to read line");
//     active_venv = input.trim().to_string();

//     match active_venv.as_str() {
//         "" => return (editor, workspace, python_path, None),
//         _ => return (editor, workspace, python_path, Some(active_venv)),
//     }
// }

// // fn 