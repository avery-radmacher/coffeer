use super::date::Date;

mod bet_generator;
mod io;

pub enum BetType {
    /// Josh wins if the balance is over the bet amount; Avery wins if the balance is below the bet amount.
    JoshOverAveryUnder,
    /// Avery wins if the balance is over the bet amount; Josh wins if the balance is below the bet amount.
    AveryOverJoshUnder,
}

pub struct Bet {
    /// The amount (in cents) to compare against the balance.
    pub bet_amount: i32,
    /// The type of bet being made.
    pub bet_type: BetType,
    /// The date upon which the bet takes effect.
    pub date: Date,
}

pub fn get_or_create_bet() -> Bet {
    // Check whether secret exists
    if let Some(secret) = io::try_get_secret() {
        secret
    } else {
        let bet = bet_generator::generate_bet();
        // TODO store bet
        bet
    }
}
