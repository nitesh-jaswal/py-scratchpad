use lexopt::*;
use lexopt::prelude::*;
use std::path::{Path, PathBuf};
use std::env;
use rand::seq::SliceRandom;
// TODO:
// * create new file in temp
// * open it in code editor
// * cleanup on close
// * 

fn get_random_four_character_id() -> String {
    let base_string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
    let id = base_string.choose_multiple(&mut rand::thread_rng(), 4).map(|x| -> u8 {*x}).collect::<Vec<_>>();
    String::from_utf8_lossy(id.as_slice()).to_string()
    // base_string
}


#[derive(Debug)]
struct Scratchpad {
        editor: PathBuf,
        venv: PathBuf,
        tmp_dir: PathBuf,
}

impl Scratchpad {
    fn parse_args() -> Self {
        use lexopt::prelude::*;

        let home = env::var("HOME").unwrap();
        let home = Path::new(home.as_str());
        let editor = PathBuf::from("/usr/local/bin/code");
        let venv = home.join(Path::new(".scratchpad/envs/DEFAULT_ENV/"));
        let tmp_dir = home.join(Path::new(".scratchpad/tmp"));
        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next().expect("Unhandled error!") {
            match arg {
                Short('n') | Long("name") => {
                    let val= parser.value().unwrap();
                    println!("name!! {}", val.to_str().unwrap());
                },
                Short('h') | Long("help") => {
                    println!("Usage: hello [-n|--number=NUM] [--shout] THING");
                    std::process::exit(0);
                },
                _ => panic!("Unexpected argument encountered {:?}", arg)
            };
        }
        Self {
            editor: editor,
            venv: venv,
            tmp_dir: tmp_dir
        }
    }
}
    
fn main() {
    // let id = get_random_four_character_id();
    let args = Scratchpad::parse_args();
    println!("{:?}", args);
}