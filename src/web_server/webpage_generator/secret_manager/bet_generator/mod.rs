use super::{Bet, BetType};
use crate::web_server::webpage_generator::date::{Date, Days};

mod random;

fn generate_bet_type() -> BetType {
    if random::uniform_bool() {
        BetType::AveryOverJoshUnder
    } else {
        BetType::JoshOverAveryUnder
    }
}

const EARLIEST_DATE: Date = Date(); // TODO
const LATEST_DATE: Date = Date(); // TODO

fn generate_bet_date() -> Date {
    // TODO should be psuedo-normally set to a date in a certain time range
    let Days(min) = EARLIEST_DATE - EARLIEST_DATE;
    let Days(max_inclusive) = LATEST_DATE - EARLIEST_DATE;
    let max = max_inclusive + 1;

    let days_after_earliest_date = Days(random::psuedo_normal(min, max, 3.0));

    todo!()
}

pub fn generate_bet() -> Bet {
    Bet {
        bet_amount: 17500, // $175 (TODO check this is correct)
        bet_type: generate_bet_type(),
        date: generate_bet_date(),
    }
}
