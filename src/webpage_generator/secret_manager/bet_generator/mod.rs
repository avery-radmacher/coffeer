use super::{Bet, BetType};
use crate::webpage_generator::date::{Date, Days};

mod random;

fn generate_bet_type() -> BetType {
    if random::uniform_bool() {
        BetType::OneUnderTwoOver
    } else {
        BetType::OneOverTwoUnder
    }
}

// TODO make statics?
fn earliest_date() -> Date {
    // TODO refine day
    Date::from_year_month_day(2024, 3, 1)
}
fn latest_date() -> Date {
    // TODO refine day
    Date::from_year_month_day(2024, 9, 1)
}

fn generate_bet_date() -> Date {
    let Days(min) = earliest_date() - earliest_date();
    let Days(max) = latest_date() - earliest_date();

    let days_after_earliest_date = Days(random::psuedo_normal(min, max, 3.0));

    earliest_date().add_days(&days_after_earliest_date)
}

pub fn generate_bet() -> Bet {
    Bet {
        parties: ("Josh".into(), "Avery".into()),
        bet_amount: 5000, // $50
        bet_type: generate_bet_type(),
        date: generate_bet_date(),
    }
}
