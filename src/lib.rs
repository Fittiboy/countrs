use chrono::{DateTime, Duration, Utc};
use std::fmt::{self, Display, Formatter};
use std::fs::{self, read_to_string};
use std::io;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Counter {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub direction: Direction,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
pub struct ConversionError;

impl std::error::Error for ConversionError {}

impl Display for ConversionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Direction {
    type Err = ConversionError;

    fn from_str(string: &str) -> Result<Self, ConversionError> {
        if string == "Up" {
            Ok(Direction::Up)
        } else if string == "Down" {
            Ok(Direction::Down)
        } else {
            Err(ConversionError)
        }
    }
}

impl Counter {
    pub fn down(start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> Counter {
        Counter {
            start: start.unwrap_or_default(),
            end: end.unwrap_or_default(),
            direction: Direction::Down,
        }
    }

    pub fn up(start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> Counter {
        Counter {
            start: start.unwrap_or_default(),
            end: end.unwrap_or_default(),
            direction: Direction::Up,
        }
    }

    pub fn to_file<T: AsRef<Path>>(&self, path: T) -> io::Result<()> {
        fs::write(
            path,
            format!(
                "{}\n{}\n{}",
                self.start.to_rfc3339(),
                self.end.to_rfc3339(),
                self.direction.to_string()
            ),
        )?;
        Ok(())
    }

    pub fn from_file<T: AsRef<Path>>(path: T) -> io::Result<Counter> {
        let lines = read_to_string(path)?;
        let mut lines = lines.split("\n");
        match (lines.next(), lines.next(), lines.next()) {
            (Some(start), Some(end), Some(direction)) => {
                let start = DateTime::parse_from_rfc3339(start)
                    .map_err(|_| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "File does not contain valid start data",
                        )
                    })?
                    .into();
                let end = DateTime::parse_from_rfc3339(end)
                    .map_err(|_| {
                        io::Error::new(
                            io::ErrorKind::InvalidData,
                            "File does not contain valid end data",
                        )
                    })?
                    .into();
                let direction = match direction.parse() {
                    Ok(direction) => direction,
                    Err(_) => {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            "File doesn ot contain valid direciton data",
                        ))
                    }
                };

                Ok(Counter {
                    start,
                    end,
                    direction,
                })
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File does not contain valid counter data",
                ))
            }
        }
    }

    pub fn flip(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        };
    }

    pub fn counter(&self) -> (i64, i64, i64) {
        let duration = match self.direction {
            Direction::Down => self.end - Utc::now(),
            Direction::Up => Utc::now() - self.start,
        };
        match duration.num_seconds() {
            num if num >= 0 => (num / 3600, num / 60 % 60, num % 60),
            _ => (0, 0, 0),
        }
    }

    pub fn try_move_start(&mut self, offset: Duration) -> Result<(), TimeOverflow> {
        if let Some(new_start) = self.start.checked_add_signed(offset) {
            self.start = new_start;
            Ok(())
        } else {
            Err(TimeOverflow)
        }
    }

    pub fn try_move_end(&mut self, offset: Duration) -> Result<(), TimeOverflow> {
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

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (hours, minutes, seconds) = self.counter();
        write!(f, "{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::Utc;

    #[test]
    fn seconds_since() {
        let counter = Counter::up(Some(Utc::now() - Duration::seconds(10)), None);
        assert_eq!(counter.to_string(), "00:00:10")
    }

    #[test]
    fn seconds_until() {
        let counter = Counter::down(None, Some(Utc::now() + Duration::seconds(10)));
        assert_eq!(counter.to_string(), "00:00:09")
    }

    #[test]
    fn minutes_since() {
        let counter = Counter::up(Some(Utc::now() - Duration::minutes(10)), None);
        assert_eq!(counter.to_string(), "00:10:00")
    }

    #[test]
    fn minutes_until() {
        let counter = Counter::down(None, Some(Utc::now() + Duration::minutes(10)));
        assert_eq!(counter.to_string(), "00:09:59")
    }

    #[test]
    fn hours_since() {
        let counter = Counter::up(Some(Utc::now() - Duration::hours(10)), None);
        assert_eq!(counter.to_string(), "10:00:00")
    }

    #[test]
    fn hours_until() {
        let counter = Counter::down(None, Some(Utc::now() + Duration::hours(10)));
        assert_eq!(counter.to_string(), "09:59:59")
    }

    #[test]
    fn days_since() {
        let counter = Counter::up(Some(Utc::now() - Duration::days(10)), None);
        assert_eq!(counter.to_string(), "240:00:00")
    }

    #[test]
    fn days_until() {
        let counter = Counter::down(None, Some(Utc::now() + Duration::days(10)));
        assert_eq!(counter.to_string(), "239:59:59")
    }

    #[test]
    fn add_time_to_down() {
        let mut counter = Counter::down(None, Some(Utc::now()));
        counter.try_move_end(Duration::seconds(10)).unwrap();
        assert_eq!(format!("{}", counter), "00:00:09")
    }

    #[test]
    fn remove_time_from_down() {
        let mut counter = Counter::down(None, Some(Utc::now() + Duration::seconds(20)));
        counter.try_move_end(Duration::seconds(-10)).unwrap();
        assert_eq!(counter.to_string(), "00:00:09")
    }

    #[test]
    fn remove_time_from_down_past_zero() {
        let mut counter = Counter::down(None, Some(Utc::now()));
        counter.try_move_end(Duration::seconds(-10)).unwrap();
        assert_eq!(counter.to_string(), "00:00:00")
    }

    #[test]
    fn add_time_to_up() {
        let mut counter = Counter::up(Some(Utc::now()), None);
        counter.try_move_start(Duration::seconds(-10)).unwrap();
        assert_eq!(counter.to_string(), "00:00:10")
    }

    #[test]
    fn remove_time_from_up() {
        let mut counter = Counter::up(Some(Utc::now() - Duration::seconds(20)), None);
        counter.try_move_start(Duration::seconds(10)).unwrap();
        assert_eq!(counter.to_string(), "00:00:10")
    }

    #[test]
    fn add_time_to_up_past_zero() {
        let mut counter = Counter::up(Some(Utc::now()), None);
        counter.try_move_start(Duration::seconds(10)).unwrap();
        assert_eq!(counter.to_string(), "00:00:00")
    }

    #[test]
    #[should_panic]
    fn too_much_time_causes_overflow() {
        let mut counter = Counter::up(None, None);
        counter.try_move_start(Duration::weeks(i64::MAX)).unwrap();
    }

    #[test]
    fn write_and_read_down() {
        let start = Utc::now();
        let end = start + Duration::days(3);

        let counter = Counter::down(Some(start), Some(end));
        counter.to_file("/tmp/counter_test_file_down.txt").unwrap();
        let read_counter = Counter::from_file("/tmp/counter_test_file_down.txt").unwrap();

        assert_eq!(counter, read_counter)
    }

    #[test]
    fn write_and_read_up() {
        let start = Utc::now();
        let end = start + Duration::days(3);

        let counter = Counter::up(Some(start), Some(end));
        counter.to_file("/tmp/counter_test_file_up.txt").unwrap();
        let read_counter = Counter::from_file("/tmp/counter_test_file_up.txt").unwrap();

        assert_eq!(counter, read_counter)
    }

    #[test]
    fn flip_up_and_down() {
        let start = Utc::now() - Duration::seconds(10);
        let end = start + Duration::seconds(20);
        let mut counter = Counter::down(Some(start), Some(end));
        assert_eq!(counter.to_string(), "00:00:09");
        counter.flip();
        assert_eq!(counter.to_string(), "00:00:10");
        counter.flip();
        assert_eq!(counter.to_string(), "00:00:09");
    }
}
