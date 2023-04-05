use chrono::{DateTime, Duration, Utc};
use std::fmt::{self, Display, Formatter};
use std::fs::{self, read_to_string};
use std::io;
use std::path::Path;

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

impl TryFrom<String> for Direction {
    type Error = ConversionError;

    fn try_from(string: String) -> Result<Self, ConversionError> {
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
            end: end.unwrap_or_default() + Duration::seconds(1), // Time is quirky
            direction: Direction::Down,
        }
    }

    pub fn up(start: Option<DateTime<Utc>>, end: Option<DateTime<Utc>>) -> Counter {
        Counter {
            start: start.unwrap_or_default(),
            end: end.unwrap_or_default() + Duration::seconds(1),
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
            (Some(start), Some(end), Some(duration)) => {
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
                let direction = match Direction::try_from(duration.to_string()) {
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

    pub fn flipped(self) -> Counter {
        let direction = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        };
        Counter {
            start: self.start,
            end: self.end,
            direction,
        }
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

impl Display for Counter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (hours, minutes, seconds) = self.counter();
        write!(f, "{:0>2}:{:0>2}:{:0>2}", hours, minutes, seconds)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
        };
        write!(f, "{}", string)
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

    #[test]
    fn add_time_to_down() {
        let mut counter = Counter::down(None, Some(Utc::now()));
        counter.move_end(Duration::seconds(10)).unwrap();
        assert_eq!(format!("{}", counter), "00:00:10")
    }

    #[test]
    fn remove_time_from_down() {
        let mut counter = Counter::down(None, Some(Utc::now() + Duration::seconds(20)));
        counter.move_end(Duration::seconds(-10)).unwrap();
        assert_eq!(format!("{}", counter), "00:00:10")
    }

    #[test]
    fn remove_time_from_down_past_zero() {
        let mut counter = Counter::down(None, Some(Utc::now()));
        counter.move_end(Duration::seconds(-10)).unwrap();
        assert_eq!(format!("{}", counter), "00:00:00")
    }

    #[test]
    fn add_time_to_up() {
        let mut counter = Counter::up(Some(Utc::now()), None);
        counter.move_start(Duration::seconds(-10)).unwrap();
        assert_eq!(format!("{}", counter), "00:00:10")
    }

    #[test]
    fn remove_time_from_up() {
        let mut counter = Counter::up(Some(Utc::now() - Duration::seconds(20)), None);
        counter.move_start(Duration::seconds(10)).unwrap();
        assert_eq!(format!("{}", counter), "00:00:10")
    }

    #[test]
    fn add_time_to_up_past_zero() {
        let mut counter = Counter::up(Some(Utc::now()), None);
        counter.move_start(Duration::seconds(10)).unwrap();
        assert_eq!(format!("{}", counter), "00:00:00")
    }

    #[test]
    #[should_panic]
    fn too_much_time_causes_overflow() {
        let mut counter = Counter::up(None, None);
        counter.move_start(Duration::weeks(i64::MAX)).unwrap();
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
}
