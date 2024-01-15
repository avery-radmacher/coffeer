use super::date::Date;
use serde::{Deserialize, Serialize};

mod bet_generator;
mod io;

#[derive(Serialize, Deserialize)]
pub enum BetType {
    /// Josh wins if the balance is over the bet amount; Avery wins if the balance is below the bet amount.
    JoshOverAveryUnder,
    /// Avery wins if the balance is over the bet amount; Josh wins if the balance is below the bet amount.
    AveryOverJoshUnder,
}

#[derive(Serialize, Deserialize)]
pub struct Bet {
    /// The amount (in cents) to compare against the balance.
    pub bet_amount: i32,
    /// The type of bet being made.
    pub bet_type: BetType,
    /// The date upon which the bet takes effect.
    pub date: Date,
}

impl Bet {
    pub fn reveal(&self) -> String {
        let amount_dollars = format!("${}", self.bet_amount / 100);
        let cents = self.bet_amount % 100;
        let amount_cents = if cents != 0 {
            format!(".{:02}", cents)
        } else {
            "".into()
        };

        let amount = format!("The bet amount was {amount_dollars}{amount_cents}.");

        let (over_winner, under_winner) = match self.bet_type {
            BetType::AveryOverJoshUnder => ("Avery", "Josh"),
            BetType::JoshOverAveryUnder => ("Josh", "Avery"),
        };

        let outcome = format!("{over_winner} wins if the remaining balance is over this amount; {under_winner} wins if the remaining balance is below this amount.");

        let maturity = format!(
            "The bet reached maturity on {}.",
            self.date.to_readable_string()
        );

        return format!("{maturity}\n{amount}\n{outcome}");
    }
}

pub fn get_or_create_bet() -> Bet {
    if let Some(secret) = io::try_get_secret() {
        secret
    } else {
        let bet = bet_generator::generate_bet();
        io::store_secret(&bet);
        bet
    }
}
