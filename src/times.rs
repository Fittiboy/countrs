use crate::errors::TimeOverflow;

pub trait Time {
    type Duration;

    fn now() -> Self;

    fn add(self, duration: Self::Duration) -> Result<Self, TimeOverflow>
    where
        Self: Sized;
}

pub trait TimeUnits {
    fn seconds(seconds: i64) -> Self;

    fn num_seconds(&self) -> i64;
}
