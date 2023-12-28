/// Represents an entire midnight-to-midnight date (year/month/day). Time zone agnostic.
pub struct Date();

/// Represents a signed duration of whole days.
pub struct Days(pub i32);

impl Date {
    /// Returns `true` iff the date represents a day in the future.
    pub fn is_future(&self) -> bool {
        true
    }

    /// Returns the days since the other date.
    pub fn days_since(&self, other: &Self) -> Days {
        todo!()
    }

    pub fn add_days(&self, days: &Days) -> Self {
        todo!()
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
