use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct InvalidDirection;

impl std::error::Error for InvalidDirection {}

impl Display for InvalidDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeParserError;

impl std::error::Error for TimeParserError {}

impl Display for TimeParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Tried to parse invalid time string")
    }
}

#[derive(Debug)]
pub struct TimeOverflow;

impl std::error::Error for TimeOverflow {}

impl Display for TimeOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Time could not be added due to an overflow")
    }
}
