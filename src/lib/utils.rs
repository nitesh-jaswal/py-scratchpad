use rand::seq::SliceRandom;
use std::{time::{SystemTime, UNIX_EPOCH}, io::Write};

pub fn get_unique_id() -> String {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).expect("Error getting timestamp");
    let timestamp = timestamp.as_secs().to_string();
    let base_string = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".as_bytes();
    let id = base_string.choose_multiple(&mut rand::thread_rng(), 4).map(|x| -> u8 {*x}).collect::<Vec<_>>();
    format!("{}_{}", String::from_utf8_lossy(id.as_slice()).to_string(), timestamp)
}