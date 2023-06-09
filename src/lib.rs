use std::fmt::{self, Display, Formatter};
use std::fs::{self, read_to_string};
use std::io;
use std::ops::Sub;
use std::path::Path;
use std::str::FromStr;

#[cfg(feature = "types")]
pub mod types;

#[cfg(feature = "chrono")]
pub mod chrono;

mod times;
pub use crate::times::*;

mod errors;
pub use crate::errors::*;

/// A counter stores `start` and `end` times, and implements `Display`
/// to either show the time passed since `start`, or until `end`,
/// formatted as `HH(+):MM:SS`.  
/// The timer will not go down past 00:00:00.
/// # Examples
/// Basic functionality is very simple:
/// ```rust
/// # use countrs::{Counter, Time, TimeUnits};
/// # use countrs::types::{Duration, TimeStamp};
/// let now = TimeStamp::now();
/// let mut counter = Counter::down(
///     Some(now - 600),
///     Some(now + 600)
/// );
///
/// // A small amount of time will have passed since `now` was assigned
/// assert_eq!(counter.to_string(), "00:09:59");
/// counter.flip();
/// // It now counts up from `start`
/// assert_eq!(counter.to_string(), "00:10:00")
/// ```
/// Both `start` and `end` times are adjustable:
/// ```rust
/// # use countrs::{Counter, Time, TimeUnits};
/// # use countrs::types::{Duration, TimeStamp};
/// let mut counter = Counter::up(Some(TimeStamp::now()), None);
/// counter.try_move_start(-30).unwrap();
///
/// assert_eq!(counter.to_string(), "00:00:30")
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Counter<T> {
    pub start: T,
    pub end: T,
    pub direction: Direction,
}

/// Specifies whether to count `Up` from a starting time,
/// or `Down` from a target end time.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    Up,
    Down,
}

impl<T, D> Counter<T>
where
    T: Copy + Default + Display + Time<Duration = D> + FromStr + Sub<T, Output = D>,
    D: TimeUnits,
{
    /// If given `None`, the default value for `T` will be assigned.
    pub fn down(start: Option<T>, end: Option<T>) -> Counter<T> {
        Counter {
            start: start.unwrap_or_default(),
            end: end.unwrap_or_default(),
            direction: Direction::Down,
        }
    }

    /// If given `None`, the default value for `T` will be assigned.
    pub fn up(start: Option<T>, end: Option<T>) -> Counter<T> {
        Counter {
            start: start.unwrap_or_default(),
            end: end.unwrap_or_default(),
            direction: Direction::Up,
        }
    }

    /// Calls `to_string` on `start`, `end`, and `direction`, and `std::fs::write`s each
    /// to one line in a file, in that order.
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        fs::write(
            path,
            format!("{}\n{}\n{}", self.start, self.end, self.direction),
        )?;
        Ok(())
    }

    /// Tries converting the first three lines of a file (read by `std::fs::read_to_string`)
    /// into a `Counter` by attempting to parse them into `start`, `end`, and `direction`
    /// respectively, calling `from_str`.
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Counter<T>> {
        let lines = read_to_string(path)?;
        let mut lines = lines.split('\n');
        if let (Some(s), Some(e), Some(d)) = (lines.next(), lines.next(), lines.next()) {
            let start = T::from_str(s).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File does not contain valid start data",
                )
            })?;
            let end = T::from_str(e).map_err(|_| {
                io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File does not contain valid end data",
                )
            })?;
            let Ok(direction) = d.parse() else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "File doesn ot contain complete direction data",
                ))
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

    /// Changes the direction of the Counter between Up/Down.
    pub fn flip(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        };
    }

    fn duration(&self) -> D {
        match self.direction {
            Direction::Down => self.end - T::now(),
            Direction::Up => T::now() - self.start,
        }
    }

    /// Returns the tuple of (hours, minutes, seconds) shown on the countdown(/up)
    pub fn counter(&self) -> (i64, i64, i64) {
        let duration = self.duration();
        match duration.num_seconds() {
            num if num >= 0 => (num / 3600, num / 60 % 60, num % 60),
            _ => (0, 0, 0),
        }
    }

    /// Returns the total number of full hours on the countdown(/up)
    pub fn hours(&self) -> i64 {
        let duration = self.duration();
        match duration.num_seconds() {
            num if num >= 0 => num / 3600,
            _ => 0,
        }
    }

    /// Returns the total number of full minutes on the countdown(/up)
    pub fn minutes(&self) -> i64 {
        let duration = self.duration();
        match duration.num_seconds() {
            num if num >= 0 => num / 60,
            _ => 0,
        }
    }

    /// Returns the total number of seconds on the countdown(/up)
    pub fn seconds(&self) -> i64 {
        let duration = self.duration();
        match duration.num_seconds() {
            num if num >= 0 => num,
            _ => 0,
        }
    }

    pub fn try_move_start(&mut self, seconds: impl Into<D>) -> Result<(), TimeOverflow> {
        self.start = self.start.add_seconds(seconds.into())?;
        Ok(())
    }

    pub fn try_move_end(&mut self, seconds: impl Into<D>) -> Result<(), TimeOverflow> {
        self.end = self.end.add_seconds(seconds.into())?;
        Ok(())
    }
}

/// "Up" -> `Up`, "Down" -> `Down`
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

/// Displayed as "HH(+):MM:SS", not below "00:00:00"
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

/// `Up` -> "Up", `Down` -> "Down"
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Up"),
            Direction::Down => write!(f, "Down"),
        }
    }
}

#[cfg(test)]
mod tests;
