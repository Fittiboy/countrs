use crate::errors::{TimeOverflow, TimeParserError};
use crate::times::Time;
use crate::TimeUnits;
use chrono::{self, DateTime, Utc};
use std::fmt::{self, Display, Formatter};
use std::ops::{Add, Sub};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy)]
pub struct TimeStamp {
    time: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration {
    duration: chrono::Duration,
}

impl Display for TimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time.to_rfc3339())
    }
}

impl FromStr for TimeStamp {
    type Err = TimeParserError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Ok(TimeStamp {
            time: DateTime::parse_from_rfc3339(string)
                .map_err(|_| TimeParserError)?
                .into(),
        })
    }
}

impl Add<Duration> for TimeStamp {
    type Output = TimeStamp;

    fn add(self, duration: Duration) -> Self::Output {
        TimeStamp {
            time: self.time + duration.duration,
        }
    }
}

impl Sub<Duration> for TimeStamp {
    type Output = TimeStamp;

    fn sub(self, duration: Duration) -> Self::Output {
        TimeStamp {
            time: self.time - duration.duration,
        }
    }
}

impl Sub<Self> for TimeStamp {
    type Output = Duration;

    fn sub(self, other: Self) -> Self::Output {
        Duration {
            duration: self.time - other.time,
        }
    }
}

impl Time for TimeStamp {
    type Duration = Duration;

    fn now() -> Self {
        TimeStamp { time: Utc::now() }
    }

    fn add(&self, duration: Duration) -> Result<TimeStamp, TimeOverflow> {
        Ok(TimeStamp {
            time: self
                .time
                .checked_add_signed(duration.duration)
                .ok_or(TimeOverflow)?,
        })
    }
}

impl TimeUnits for Duration {
    fn seconds(seconds: i64) -> Self {
        Duration {
            duration: chrono::Duration::seconds(seconds),
        }
    }

    fn num_seconds(&self) -> i64 {
        self.duration.num_seconds()
    }
}
