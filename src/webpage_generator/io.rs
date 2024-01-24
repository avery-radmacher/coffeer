use crate::webpage_generator::secret_manager::Bet;
use serde::de::DeserializeOwned;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::str::FromStr;

fn get_path() -> PathBuf {
    PathBuf::from_str(".\\data\\bet.json").expect("Error creating path")
}

fn read_json_file<T>(path: &PathBuf) -> T
where
    T: DeserializeOwned,
{
    let file = File::open(path).expect("Error opening file");
    let item: T = serde_json::from_reader(file).expect("Error reading file");
    item
}

pub fn try_get_secret() -> Option<Bet> {
    let path = get_path();
    if !path.exists() {
        return None;
    }

    Some(read_json_file(&path))
}

pub fn store_secret(bet: &Bet) {
    let path = get_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Error creating storage directory");
    }
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .expect("Error creating new file");

    serde_json::to_writer_pretty(file, bet).expect("Error writing secret");
}
