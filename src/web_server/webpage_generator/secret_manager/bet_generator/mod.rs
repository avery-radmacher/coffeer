use self::random::uniform_bool;

use super::{Bet, BetType};
use crate::web_server::webpage_generator::date::Date;

mod random;

fn generate_bet_type() -> BetType {
    if uniform_bool() {
        BetType::AveryOverJoshUnder
    } else {
        BetType::JoshOverAveryUnder
    }
}

const EARLIEST_DATE: Date = Date(); // TODO
const LATEST_DATE: Date = Date(); // TODO

fn generate_bet_date() -> Date {
    // TODO should be psuedo-normally set to a date in a certain time range
    todo!()
}

pub fn generate_bet() -> Bet {
    Bet {
        bet_amount: 17500, // $175 (TODO check this is correct)
        bet_type: generate_bet_type(),
        date: generate_bet_date(),
    }
}
