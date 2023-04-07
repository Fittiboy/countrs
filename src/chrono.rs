//! This module implements the `Time` and `TimeUnits` traits
//! for `chrono`'s `DateTime<Tz>`, as long as `From<DateTime<Utc>>`
//! is implemented for `DateTime<Tz>`.
use crate::errors::TimeOverflow;
use crate::{Time, TimeUnits};
use chrono::{DateTime, Duration, TimeZone, Utc};
use std::ops::Add;

impl<Tz: TimeZone + 'static> Time for DateTime<Tz>
where
    &'static DateTime<Tz>: Add<Duration, Output = DateTime<Tz>>,
    DateTime<Tz>: From<DateTime<Utc>>,
{
    type Duration = Duration;

    fn now() -> Self {
        Utc::now().into()
    }

    fn add_seconds(self, duration: Self::Duration) -> Result<Self, TimeOverflow>
    where
        Self: Sized,
    {
        Ok(self + duration)
    }
}

impl TimeUnits for Duration {
    fn seconds(seconds: i64) -> Self {
        Duration::seconds(seconds)
    }

    fn num_seconds(&self) -> i64 {
        self.num_seconds()
    }
}
