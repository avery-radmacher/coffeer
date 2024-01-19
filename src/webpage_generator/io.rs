use crate::webpage_generator::secret_manager::Bet;
use std::fs::{self, File, OpenOptions};
use std::path::PathBuf;
use std::str::FromStr;

fn get_path() -> PathBuf {
    PathBuf::from_str(".\\data\\bet.json").expect("Error creating path")
}

pub fn try_get_secret() -> Option<Bet> {
    let path = get_path();
    if !path.exists() {
        return None;
    }

    let file = File::open(path).expect("Error opening file");
    let bet = serde_json::from_reader(file).expect("Error reading file");
    Some(bet)
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
