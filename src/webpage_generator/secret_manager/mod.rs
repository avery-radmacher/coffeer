use super::date::Date;
use crate::io;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod bet_generator;

#[derive(Serialize, Deserialize)]
pub enum BetType {
    /// The first part to the bet wins if the balance is above the bet amount; the second party to the bet wins if the balance is below the bet amount.
    OneOverTwoUnder,
    /// The first part to the bet wins if the balance is below the bet amount; the second party to the bet wins if the balance is above the bet amount.
    OneUnderTwoOver,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bet {
    /// The parties to the bet.
    pub parties: (String, String),
    /// The amount (in cents) to compare against the balance.
    pub bet_amount: i32,
    /// The type of bet being made.
    pub bet_type: BetType,
    /// The date upon which the bet takes effect.
    pub date: Date,
}

impl Bet {
    pub fn divulge_if_mature(self) -> Option<Self> {
        if !self.date.is_future() {
            Some(self)
        } else {
            None
        }
    }
}

fn get_path() -> PathBuf {
    PathBuf::from(".\\data\\bet.json")
}

fn try_get_secret() -> Option<Bet> {
    let path = get_path();
    if !path.exists() {
        return None;
    }

    Some(io::read_json_file(&path))
}

fn store_secret(bet: &Bet) {
    io::write_json_file(&get_path(), bet)
}

pub fn get_or_create_bet() -> Bet {
    if let Some(secret) = try_get_secret() {
        secret
    } else {
        let bet = bet_generator::generate_bet();
        store_secret(&bet);
        bet
    }
}
