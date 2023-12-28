use super::date::Date;

mod io;
mod secret_generator;

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
    Bet {
        bet_amount: 17500,                     // $175 (TODO check this is correct)
        bet_type: BetType::AveryOverJoshUnder, // TODO should be randomly set to one of two choices
        date: Date(), // TODO should be psuedo-normally set to a date in a certain time range
    }
}
