use self::random::uniform_bool;

use super::{Bet, BetType};
use crate::web_server::webpage_generator::date::Date;

mod random;

pub fn generate_bet() -> Bet {
    Bet {
        bet_amount: 17500, // $175 (TODO check this is correct)
        bet_type: if uniform_bool() {
            BetType::AveryOverJoshUnder
        } else {
            BetType::JoshOverAveryUnder
        },
        date: Date(), // TODO should be psuedo-normally set to a date in a certain time range
    }
}
