use serde::{de::DeserializeOwned, ser::Serialize};
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;

pub fn read_json_file<T: DeserializeOwned>(path: &PathBuf) -> T {
    let file = File::open(path).expect("Error opening file");
    let item: T = serde_json::from_reader(file).expect("Error reading file");
    item
}

pub fn write_json_file<T: Serialize>(path: &PathBuf, value: &T) {
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
