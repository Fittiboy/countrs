use crate::errors::TimeOverflow;

pub trait Time {
    type Duration;

    fn now() -> Self;

    fn add(&self, duration: Self::Duration) -> Result<Self, TimeOverflow>
    where
        Self: Sized;
}

pub trait TimeUnits {
    fn seconds(seconds: i64) -> Self;

    fn minutes(minutes: i64) -> Self;

    fn hours(hours: i64) -> Self;

    fn days(days: i64) -> Self;

    fn weeks(weeks: i64) -> Self;

    fn num_seconds(&self) -> i64;
}
