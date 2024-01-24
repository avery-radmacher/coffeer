use crate::webpage_generator::secret_manager::Bet;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::str::FromStr;

fn get_path() -> PathBuf {
    PathBuf::from_str(".\\data\\bet.json").expect("Error creating path")
}

fn read_json_file<T: DeserializeOwned>(path: &PathBuf) -> T {
    let file = File::open(path).expect("Error opening file");
    let item: T = serde_json::from_reader(file).expect("Error reading file");
    item
}

fn write_json_file<T: Serialize>(path: &PathBuf, value: &T) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Error creating storage directory");
    }
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Error creating new file");

    serde_json::to_writer_pretty(file, value).expect("Error writing file");
}

pub fn try_get_secret() -> Option<Bet> {
    let path = get_path();
    if !path.exists() {
        return None;
    }

    Some(read_json_file(&path))
}

pub fn store_secret(bet: &Bet) {
    write_json_file(&get_path(), bet)
}
