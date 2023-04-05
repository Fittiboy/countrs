use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeParserError;

impl std::error::Error for TimeParserError {}

impl Display for TimeParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "Tried to parse invalid time string")
    }
}

#[derive(Debug)]
pub struct TimeOverflow;

impl std::error::Error for TimeOverflow {}

impl Display for TimeOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", "Time could not be added due to an overflow")
    }
}

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
