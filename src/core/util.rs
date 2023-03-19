use colored::*;
use rand::seq::SliceRandom;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{BufRead, Write};

pub fn get_unique_id() -> String {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error getting timestamp");
    let timestamp = timestamp.as_secs().to_string();
    let base_string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
    let id = base_string.choose_multiple(&mut rand::thread_rng(), 4).map(|x| -> u8 {*x}).collect::<Vec<_>>();
    format!("{}_{}", String::from_utf8_lossy(id.as_slice()).to_string(), timestamp)
}

fn prompt<R, W>(mut reader: R, mut writer: W, prompt_str: &str, prompt_help: &str, default: &str) -> Option<String>
where
    R: BufRead,
    W: Write,
{
    let mut input = String::new();
    write!(&mut writer, "{}\n", prompt_help.red()).expect("Unable to write to terminal!");
    write!(&mut writer, "{}", prompt_str.green()).expect("Unable to write to terminal!");
    reader.read_line(&mut input).expect("Failed to read line");
    input = input.trim().to_string();
    match input.as_str() {
        "" => {
            if default != "" {
                return Some(default.to_string())
            }
            return None
        },
        val => return Some(val.to_string())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;


    #[rstest]
    fn test_get_unique_id() {
        // Arrange
        let valid_ascii = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".split("").collect::<Vec<_>>();
        let unique_id = get_unique_id();
        // Act
        let split_id: Vec<&str> = unique_id.split("_").collect();
        // Assert
        assert_eq!(split_id.len(), 2usize);
        let ascii_str  = split_id[0].split("").collect::<Vec<_>>();
        let time  = split_id[1].parse::<u32>();
        assert_eq!(time.is_ok(), true);
        assert_eq!(ascii_str.iter().all(|c| {valid_ascii.contains(c)}), true);
    }

    #[rstest]
    fn test_prompt_valid_response() {
        // Arrange
        let valid_response = b"valid_response";
        // Act
        let mut output = Vec::new();
        let answer = prompt(&valid_response[..], &mut output, "Enter some valid input: ", "This will help you.", "default");
        let output = String::from_utf8(output).expect("Not UTF-8");
        // Assert
        assert_eq!(format!("{}\n{}", "This will help you.".red(), "Enter some valid input: ".green()), output);
        assert_eq!(Some(String::from_utf8(valid_response.to_vec()).unwrap()), answer);
    }
    
    #[rstest]
    fn test_prompt_default_response() {
        // Arrange
        let default_response = b"";
        // Act
        let mut output = Vec::new();
        let answer = prompt(&default_response[..], &mut output, "Enter some valid input: ", "This will help you.", "default");
        let output = String::from_utf8(output).expect("Not UTF-8");
        // Assert
        assert_eq!(format!("{}\n{}", "This will help you.".red(), "Enter some valid input: ".green()), output);
        assert_eq!(Some("default".to_string()), answer);
    }
    
    #[rstest]
    fn test_prompt_no_response() {
        // Arrange
        let no_response = b"";
        // Act
        let mut output = Vec::new();
        let answer = prompt(&no_response[..], &mut output, "Enter some valid input: ", "This will help you.", "");
        let output = String::from_utf8(output).expect("Not UTF-8");
        // Assert
        assert_eq!(format!("{}\n{}", "This will help you.".red(), "Enter some valid input: ".green()), output);
        assert_eq!(None, answer);
    }
}