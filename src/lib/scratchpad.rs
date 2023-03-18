use std::fs::DirEntry;
use std::convert::From;
use std::io::Write;
use std::path::{PathBuf, Path};
use crate::utils::get_unique_id;

struct ConfigData {
    editor: PathBuf,
    workspace: PathBuf,
    python_path: PathBuf,
    active_venv: PathBuf,
    last_created_file: Option<PathBuf>
}

// impl default trait
// impl new function
// impl new function
struct Venv {

}

// Maps to index.json in root of source
// index_file_path: PathBuf,
// num_files:
struct FileIndexEntry {
    id: String,
    description: String,
    date_created: String
}   

impl FileIndexEntry {
    fn new(id: &str, description: &str, date_created: &str) -> Self {
        FileIndexEntry { 
            id: id.to_string(), 
            description: description.to_string(), 
            date_created: date_created.to_string() 
        }
    }
}

struct FileIndex {
    index_path: PathBuf,
    item_list: Vec<FileIndexEntry>,
}

impl FileIndex {
    fn added_a_file(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn removed_file(&mut self) -> std::io::Result<()> {
        Ok(())
    }
    fn update_index(&self) -> std::io::Result<()>{
        
        Ok(())
    }
}

impl Iterator for FileIndex {
    type Item = FileIndexEntry;
    fn next(&mut self) -> Option<Self::Item> {
        self.item_list.pop()
    }
}
// impl maintaining and updating of index of files that exist
struct ScratchpadBhandaar<'a> {
    bhandaar_path: PathBuf,
    num_files: &'a mut u16,
    last_created_file: Option<&'a mut PathBuf>,
    index: &'a mut FileIndex
}

impl<'a> ScratchpadBhandaar<'a> {

    fn get_number_files(bhandaar_path: &Path) -> Option<u16> {
        if bhandaar_path.is_dir() {
            let num_file = bhandaar_path
                .read_dir()
                .expect(format!("Cannot read directory: {}", bhandaar_path.display()).as_str())
                .filter(|&file| -> bool {
                    match file {
                        Ok(val) => {
                            if let Some(file_name) = val.file_name().to_str() {
                                return file_name.contains("scratchpad")
                            }
                            else {
                                println!("Unable to read file name in directory: {}", bhandaar_path.display());
                                return false
                            }
                        },
                        Err(err) => {
                            println!("Panicked while trying to read directory: {}", bhandaar_path.display());
                            return false
                        }
                    }
                })
                .count();
            return Some(num_file as u16)
        }
        else {
            println!("Path to directory not found: {}", bhandaar_path.display());
            return None
        }
    }

    // Return file_path of deleted files
    fn create_new_file(&mut self) {
        let file_name = format!("scratchpad_{}", get_unique_id()).as_str();
        let file_path = self.bhandaar_path.join(file_name);
        let new_file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&file_path);
        match new_file {
            Ok(new_file) => {
                let default_content = "import os\nimport sys\n\ndef main(args):\n\t...\n\nif __name__ == \"__main__\":\n\tmain(sys.argv)";
                new_file.write_all(default_content.as_bytes());
            },
            Err(err) => panic!("Fatal Error! Unable to create new file: {:?}", err)
        }
        self.last_created_file = Some(file_path).as_mut();
        self.index.added_a_file();
        self.num_files = &mut (*self.num_files + 1);
    }

    // TODO: modify delete command to accept Vec of file_names/patterns
    fn delete_file(&mut self, partial_id: &str) -> Option<&str> {
        if *self.num_files == 0u16 {
            println!("No files available to delete!");
            return None
        }

        let deleted_file_path = self.bhandaar_path
            .read_dir()
            .expect(format!("Cannot read directory: {}", self.bhandaar_path.display()).as_str())
            .filter_map(|file| -> Option<DirEntry> {
                match file {
                    Ok(dir_entry) => Some(dir_entry),
                    Err(err) => {
                        println!("Cannot read file {}: {}", self.bhandaar_path.display(), err);
                        None
                    }
                }
            })
            .find(|&file| -> bool {
                if let Some(file_name) = file.file_name().to_str() {
                    return file_name.contains(partial_id)
                }
                else {
                    println!("Unable to read file name in directory: {}", self.bhandaar_path.display());
                    return false
                }
            });

        match deleted_file_path {
            Some(file_path) => {
                let file_path = file_path.path();
                match std::fs::remove_file(file_path) {
                    Ok(_) => {
                        self.index.removed_file();
                        return Some(file_path.to_str().unwrap())
                    },
                    Err(err) => {
                        println!("Unable to delete file {}: {:?}", file_path.display(), err);
                        None
                    }
                }
            },
            None => {
                println!("No file found matching the id {}", partial_id);
                None
            }
        }
    }

    fn list_all_files(&mut self) {
        // finds all "scratchpad" files
        // updates index
    }
}

// impl From<ConfigData> for ScratchpadBhandaar {
//     fn from(config_data: ConfigData) -> Self {
//         config_data.active_venv
//     }
// }

