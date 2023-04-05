use std::fmt::{self, Display, Formatter};
use std::fs::{self, read_to_string};
use std::io;
use std::ops::Sub;
use std::path::Path;
use std::str::FromStr;

#[cfg(feature = "chrono")]
pub mod chrono;

mod times;
pub use crate::times::*;

mod errors;
pub use crate::errors::*;

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
mod tests;
