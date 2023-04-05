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

impl Display for TimeStamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.time.to_rfc3339())
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

impl TimeStamp {
    pub fn now() -> Self {
        TimeStamp { time: Utc::now() }
    }

    pub fn add(&self, duration: Duration) -> Result<TimeStamp, TimeOverflow> {
        Ok(TimeStamp {
            time: self
                .time
                .checked_add_signed(duration.duration)
                .ok_or(TimeOverflow)?,
        })
    }
}

impl Duration {
    pub fn seconds(seconds: i64) -> Self {
        Duration {
            duration: chrono::Duration::seconds(seconds),
        }
    }

    pub fn minutes(minutes: i64) -> Self {
        Duration {
            duration: chrono::Duration::minutes(minutes),
        }
    }

    pub fn hours(hours: i64) -> Self {
        Duration {
            duration: chrono::Duration::hours(hours),
        }
    }

    pub fn days(days: i64) -> Self {
        Duration {
            duration: chrono::Duration::days(days),
        }
    }

    pub fn weeks(weeks: i64) -> Self {
        Duration {
            duration: chrono::Duration::weeks(weeks),
        }
    }

    pub fn num_seconds(&self) -> i64 {
        self.duration.num_seconds()
    }
}
