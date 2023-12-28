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

const EARLIEST_DATE: Date = Date::from_year_month_day(2024, 3, 1); // TODO refine day
const LATEST_DATE: Date = Date::from_year_month_day(2024, 9, 1); // TODO refine day

fn generate_bet_date() -> Date {
    let Days(min) = EARLIEST_DATE - EARLIEST_DATE;
    let Days(max) = LATEST_DATE - EARLIEST_DATE;

    let days_after_earliest_date = Days(random::psuedo_normal(min, max, 3.0));

    EARLIEST_DATE.add_days(&days_after_earliest_date)
}

pub fn generate_bet() -> Bet {
    Bet {
        bet_amount: 17500, // $175 (TODO check this is correct)
        bet_type: generate_bet_type(),
        date: generate_bet_date(),
    }
}
