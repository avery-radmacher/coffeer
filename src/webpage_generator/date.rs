use date_time::date_tuple::DateTuple;
use serde::{Deserialize, Serialize};

/// Represents an entire midnight-to-midnight date (year/month/day). Time zone agnostic.

#[derive(Serialize, Deserialize)]
pub struct Date(DateTuple);

/// Represents a signed duration of whole days.
pub struct Days(pub i32);

impl Date {
    // Constructs a `Date` to represent the year-month-day triple. Panics on invalid input.
    pub fn from_year_month_day(year: u32, month: u32, day: u32) -> Self {
        Date(
            DateTuple::new(year as u16, month as u8, day as u8)
                .expect("Attempted to construct invalid Date"),
        )
    }

    /// Returns `true` iff the date represents a day in the future.
    pub fn is_future(&self) -> bool {
        self.0 > DateTuple::today()
    }

    /// Returns the days since the other date.
    pub fn days_since(&self, other: &Self) -> Days {
        Days(self.0.to_days() as i32 - other.0.to_days() as i32)
    }

    pub fn add_days(&self, days: &Days) -> Self {
        let mut date = self.0.clone();
        date.add_days(days.0 as u32);
        Date(date)
    }
}

impl std::ops::Add<Days> for Date {
    type Output = Self;
    fn add(self, rhs: Days) -> Self::Output {
        self.add_days(&rhs)
    }
}

impl std::ops::Sub for Date {
    type Output = Days;
    fn sub(self, rhs: Self) -> Self::Output {
        self.days_since(&rhs)
    }
}
