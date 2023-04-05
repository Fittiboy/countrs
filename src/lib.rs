use chrono::{DateTime, Duration, Utc};
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Counter<T> {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub direction: PhantomData<T>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Up;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Down;

pub trait Count {
    fn count_directionless(&self) -> (i64, i64, i64);
}

impl Counter<Down> {
    pub fn down(start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> Counter<Down> {
        Counter {
            start: start.unwrap_or_default(),
            end: end.unwrap_or_default() + Duration::seconds(1), // Time is quirky
            direction: PhantomData::<Down>,
        }
    }

    pub fn flipped(self) -> Counter<Up> {
        Counter {
            start: self.start,
            end: self.end,
            direction: PhantomData::<Up>,
        }
    }

    pub fn until(&self) -> (i64, i64, i64) {
        hms(self.end - Utc::now())
    }
}

impl Counter<Up> {
    pub fn up(start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> Counter<Up> {
        Counter {
            start: start.unwrap_or_default(),
            end: end.unwrap_or_default(),
            direction: PhantomData::<Up>,
        }
    }

    pub fn flipped(self) -> Counter<Down> {
        Counter {
            start: self.start,
            end: self.end,
            direction: PhantomData::<Down>,
        }
    }

    pub fn since(&self) -> (i64, i64, i64) {
        hms(Utc::now() - self.start)
    }
}

fn hms(duration: Duration) -> (i64, i64, i64) {
    let total_seconds = duration.num_seconds();
    let seconds = total_seconds % 60;
    let minutes = total_seconds / 60 % 60;
    let hours = total_seconds / 3600;
    (hours, minutes, seconds)
}

impl<T> Counter<T> {
    pub fn move_start(&mut self, offset: Duration) -> Result<(), TimeOverflow> {
        if let Some(new_start) = self.start.checked_add_signed(offset) {
            self.start = new_start;
            Ok(())
        } else {
            Err(TimeOverflow)
        }
    }

    pub fn move_end(&mut self, offset: Duration) -> Result<(), TimeOverflow> {
        if let Some(new_end) = self.end.checked_add_signed(offset) {
            self.end = new_end;
            Ok(())
        } else {
            Err(TimeOverflow)
        }
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

impl Count for Counter<Down> {
    fn count_directionless(&self) -> (i64, i64, i64) {
        self.until()
    }
}

impl Count for Counter<Up> {
    fn count_directionless(&self) -> (i64, i64, i64) {
        self.since()
    }
}

impl<T> Display for Counter<T>
where
    Self: Count,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (hours, minutes, seconds) = self.count_directionless();
        write!(f, "{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds,)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::Utc;

    #[test]
    fn seconds_since() {
        let counter = Counter::up(Some(Utc::now() - Duration::seconds(10)), None);
        assert_eq!(format!("{}", counter), "00:00:10")
    }

    #[test]
    fn seconds_until() {
        let counter = Counter::down(None, Some(Utc::now() + Duration::seconds(10)));
        assert_eq!(format!("{}", counter), "00:00:10")
    }

    #[test]
    fn minutes_since() {
        let counter = Counter::up(Some(Utc::now() - Duration::minutes(10)), None);
        assert_eq!(format!("{}", counter), "00:10:00")
    }

    #[test]
    fn minutes_until() {
        let counter = Counter::down(None, Some(Utc::now() + Duration::minutes(10)));
        assert_eq!(format!("{}", counter), "00:10:00")
    }

    #[test]
    fn hours_since() {
        let counter = Counter::up(Some(Utc::now() - Duration::hours(10)), None);
        assert_eq!(format!("{}", counter), "10:00:00")
    }

    #[test]
    fn hours_until() {
        let counter = Counter::down(None, Some(Utc::now() + Duration::hours(10)));
        assert_eq!(format!("{}", counter), "10:00:00")
    }

    #[test]
    fn days_since() {
        let counter = Counter::up(Some(Utc::now() - Duration::days(10)), None);
        assert_eq!(format!("{}", counter), "240:00:00")
    }

    #[test]
    fn days_until() {
        let counter = Counter::down(None, Some(Utc::now() + Duration::days(10)));
        assert_eq!(format!("{}", counter), "240:00:00")
    }
}
