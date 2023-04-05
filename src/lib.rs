use crate::times::{Time, TimeOverflow, TimeUnits};
use std::fmt::{self, Display, Formatter};
use std::fs::{self, read_to_string};
use std::io;
use std::ops::Sub;
use std::path::Path;
use std::str::FromStr;

pub mod times;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Counter<T> {
    pub start: T,
    pub end: T,
    pub direction: Direction,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
pub struct InvalidDirection;

impl std::error::Error for InvalidDirection {}

impl Display for InvalidDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for Direction {
    type Err = InvalidDirection;

    fn from_str(string: &str) -> Result<Self, InvalidDirection> {
        match string {
            "Up" => Ok(Direction::Up),
            "Down" => Ok(Direction::Down),
            _ => Err(InvalidDirection),
        }
    }
}

impl<T, D> Counter<T>
where
    T: Copy + Default + Display + Time<Duration = D> + FromStr + Sub<T, Output = D>,
    D: TimeUnits,
{
    pub fn down(start: Option<T>, end: Option<T>) -> Counter<T> {
        Counter {
            start: start.unwrap_or_default(),
            end: end.unwrap_or_default(),
            direction: Direction::Down,
        }
    }

    pub fn up(start: Option<T>, end: Option<T>) -> Counter<T> {
        Counter {
            start: start.unwrap_or_default(),
            end: end.unwrap_or_default(),
            direction: Direction::Up,
        }
    }

    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::write(
            path,
            format!(
                "{}\n{}\n{}",
                self.start.to_string(),
                self.end.to_string(),
                self.direction.to_string()
            ),
        )?;
        Ok(())
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Counter<T>> {
        let lines = read_to_string(path)?;
        let mut lines = lines.split("\n");
        if let (Some(s), Some(e), Some(d)) = (lines.next(), lines.next(), lines.next()) {
            let start = T::from_str(s)
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "File does not contain valid start data",
                    )
                })?
                .into();
            let end = T::from_str(e)
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "File does not contain valid end data",
                    )
                })?
                .into();
            let direction = match d.parse() {
                Ok(direction) => direction,
                Err(_) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "File doesn ot contain complete direction data",
                    ))
                }
            };

            return Ok(Counter {
                start,
                end,
                direction,
            });
        }
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "File does not contain valid counter data",
        ))
    }

    pub fn flip(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        };
    }

    pub fn counter(&self) -> (i64, i64, i64) {
        let duration = match self.direction {
            Direction::Down => self.end - T::now(),
            Direction::Up => T::now() - self.start,
        };
        match duration.num_seconds() {
            num if num >= 0 => (num / 3600, num / 60 % 60, num % 60),
            _ => (0, 0, 0),
        }
    }

    pub fn try_move_start(&mut self, offset: D) -> Result<(), TimeOverflow> {
        self.start = self.start.add(offset)?;
        Ok(())
    }

    pub fn try_move_end(&mut self, offset: D) -> Result<(), TimeOverflow> {
        self.end = self.end.add(offset)?;
        Ok(())
    }
}

impl<T, D> Display for Counter<T>
where
    T: Copy + Default + Display + Time<Duration = D> + FromStr + Sub<T, Output = D>,
    D: TimeUnits,
{
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
    use crate::times::*;
    use crate::*;

    #[test]
    fn seconds_since() {
        let counter = Counter::up(Some(TimeStamp::now() - Duration::seconds(10)), None);
        assert_eq!(counter.to_string(), "00:00:10")
    }

    #[test]
    fn seconds_until() {
        let counter = Counter::down(None, Some(TimeStamp::now() + Duration::seconds(10)));
        assert_eq!(counter.to_string(), "00:00:09")
    }

    #[test]
    fn minutes_since() {
        let counter = Counter::up(Some(TimeStamp::now() - Duration::minutes(10)), None);
        assert_eq!(counter.to_string(), "00:10:00")
    }

    #[test]
    fn minutes_until() {
        let counter = Counter::down(None, Some(TimeStamp::now() + Duration::minutes(10)));
        assert_eq!(counter.to_string(), "00:09:59")
    }

    #[test]
    fn hours_since() {
        let counter = Counter::up(Some(TimeStamp::now() - Duration::hours(10)), None);
        assert_eq!(counter.to_string(), "10:00:00")
    }

    #[test]
    fn hours_until() {
        let counter = Counter::down(None, Some(TimeStamp::now() + Duration::hours(10)));
        assert_eq!(counter.to_string(), "09:59:59")
    }

    #[test]
    fn days_since() {
        let counter = Counter::up(Some(TimeStamp::now() - Duration::days(10)), None);
        assert_eq!(counter.to_string(), "240:00:00")
    }

    #[test]
    fn days_until() {
        let counter = Counter::down(None, Some(TimeStamp::now() + Duration::days(10)));
        assert_eq!(counter.to_string(), "239:59:59")
    }

    #[test]
    fn add_time_to_down() {
        let mut counter = Counter::down(None, Some(TimeStamp::now()));
        counter.try_move_end(Duration::seconds(10)).unwrap();
        assert_eq!(format!("{}", counter), "00:00:09")
    }

    #[test]
    fn remove_time_from_down() {
        let mut counter = Counter::down(None, Some(TimeStamp::now() + Duration::seconds(20)));
        counter.try_move_end(Duration::seconds(-10)).unwrap();
        assert_eq!(counter.to_string(), "00:00:09")
    }

    #[test]
    fn remove_time_from_down_past_zero() {
        let mut counter = Counter::down(None, Some(TimeStamp::now()));
        counter.try_move_end(Duration::seconds(-10)).unwrap();
        assert_eq!(counter.to_string(), "00:00:00")
    }

    #[test]
    fn add_time_to_up() {
        let mut counter = Counter::up(Some(TimeStamp::now()), None);
        counter.try_move_start(Duration::seconds(-10)).unwrap();
        assert_eq!(counter.to_string(), "00:00:10")
    }

    #[test]
    fn remove_time_from_up() {
        let mut counter = Counter::up(Some(TimeStamp::now() - Duration::seconds(20)), None);
        counter.try_move_start(Duration::seconds(10)).unwrap();
        assert_eq!(counter.to_string(), "00:00:10")
    }

    #[test]
    fn add_time_to_up_past_zero() {
        let mut counter = Counter::up(Some(TimeStamp::now()), None);
        counter.try_move_start(Duration::seconds(10)).unwrap();
        assert_eq!(counter.to_string(), "00:00:00")
    }

    #[test]
    #[should_panic]
    fn too_much_time_causes_overflow() {
        let mut counter = Counter::<TimeStamp>::up(None, None);
        counter.try_move_start(Duration::weeks(i64::MAX)).unwrap();
    }

    #[test]
    fn write_and_read_down() {
        let start = TimeStamp::now();
        let end = start + Duration::days(3);

        let counter = Counter::down(Some(start), Some(end));
        counter.to_file("/tmp/counter_test_file_down.txt").unwrap();
        let read_counter = Counter::from_file("/tmp/counter_test_file_down.txt").unwrap();

        assert_eq!(counter, read_counter)
    }

    #[test]
    fn write_and_read_up() {
        let start = TimeStamp::now();
        let end = start + Duration::days(3);

        let counter = Counter::up(Some(start), Some(end));
        counter.to_file("/tmp/counter_test_file_up.txt").unwrap();
        let read_counter = Counter::from_file("/tmp/counter_test_file_up.txt").unwrap();

        assert_eq!(counter, read_counter)
    }

    #[test]
    fn flip_up_and_down() {
        let start = TimeStamp::now() - Duration::seconds(10);
        let end = start + Duration::seconds(20);
        let mut counter = Counter::down(Some(start), Some(end));
        assert_eq!(counter.to_string(), "00:00:09");
        counter.flip();
        assert_eq!(counter.to_string(), "00:00:10");
        counter.flip();
        assert_eq!(counter.to_string(), "00:00:09");
    }
}
