use crate::*;

impl TimeUnits for i64 {
    fn num_seconds(&self) -> i64 {
        *self
    }

    fn seconds(seconds: i64) -> i64 {
        seconds
    }
}

impl Time for i64 {
    type Duration = i64;

    fn now() -> Self {
        0
    }

    fn add_seconds(self, duration: Self::Duration) -> Result<Self, TimeOverflow> {
        Ok(self + duration)
    }
}

#[test]
fn seconds_since() {
    let counter = Counter::up(Some(-10), None);
    assert_eq!(counter.to_string(), "00:00:10")
}

#[test]
fn seconds_until() {
    let counter = Counter::down(None, Some(10));
    assert_eq!(counter.to_string(), "00:00:10")
}

#[test]
fn minutes_since() {
    let counter = Counter::up(Some(-60 * 10), None);
    assert_eq!(counter.to_string(), "00:10:00")
}

#[test]
fn minutes_until() {
    let counter = Counter::down(None, Some(60 * 10));
    assert_eq!(counter.to_string(), "00:10:00")
}

#[test]
fn hours_since() {
    let counter = Counter::up(Some(-3600 * 10), None);
    assert_eq!(counter.to_string(), "10:00:00")
}

#[test]
fn hours_until() {
    let counter = Counter::down(None, Some(3600 * 10));
    assert_eq!(counter.to_string(), "10:00:00")
}

#[test]
fn days_since() {
    let counter = Counter::up(Some(-86400 * 10), None);
    assert_eq!(counter.to_string(), "240:00:00")
}

#[test]
fn days_until() {
    let counter = Counter::down(None, Some(86400 * 10));
    assert_eq!(counter.to_string(), "240:00:00")
}

#[test]
fn hours_minutes_seconds() {
    let counter = Counter::down(None, Some(100));
    assert_eq!(
        counter.to_string(),
        format!(
            "{:0>2}:{:0>2}:{:0>2}",
            counter.hours(),
            counter.minutes() % 60,
            counter.seconds() % 60,
        )
    )
}

#[test]
fn add_time_to_down() {
    let mut counter = Counter::down(None, Some(0));
    counter.try_move_end(10).unwrap();
    assert_eq!(format!("{}", counter), "00:00:10")
}

#[test]
fn remove_time_from_down() {
    let mut counter = Counter::down(None, Some(20));
    counter.try_move_end(i64::seconds(-10)).unwrap();
    assert_eq!(counter.to_string(), "00:00:10")
}

#[test]
fn remove_time_from_down_past_zero() {
    let mut counter = Counter::down(None, Some(0));
    counter.try_move_end(i64::seconds(-10)).unwrap();
    assert_eq!(counter.to_string(), "00:00:00")
}

#[test]
fn add_time_to_up() {
    let mut counter = Counter::up(Some(0), None);
    counter.try_move_start(i64::seconds(-10)).unwrap();
    assert_eq!(counter.to_string(), "00:00:10")
}

#[test]
fn remove_time_from_up() {
    let mut counter = Counter::up(Some(-20), None);
    counter.try_move_start(10).unwrap();
    assert_eq!(counter.to_string(), "00:00:10")
}

#[test]
fn add_time_to_up_past_zero() {
    let mut counter = Counter::up(Some(0), None);
    counter.try_move_start(10).unwrap();
    assert_eq!(counter.to_string(), "00:00:00")
}

#[test]
#[should_panic]
fn too_much_time_causes_overflow() {
    let mut counter = Counter::<i64>::up(None, None);
    counter.try_move_start(i64::MAX).unwrap();
    counter.try_move_start(1).unwrap();
}

#[test]
fn write_and_read_down() {
    let start = 0;
    let end = start + 86400 * 3;

    let counter = Counter::down(Some(start), Some(end));
    counter.to_file("/tmp/counter_test_file_down.txt").unwrap();
    let read_counter = Counter::from_file("/tmp/counter_test_file_down.txt").unwrap();

    assert_eq!(counter, read_counter)
}

#[test]
fn write_and_read_up() {
    let start = 0;
    let end = start + 86400 * 3;

    let counter = Counter::up(Some(start), Some(end));
    counter.to_file("/tmp/counter_test_file_up.txt").unwrap();
    let read_counter = Counter::from_file("/tmp/counter_test_file_up.txt").unwrap();

    assert_eq!(counter, read_counter)
}

#[test]
fn flip_up_and_down() {
    let start = -10;
    let end = start + 20;
    let mut counter = Counter::down(Some(start), Some(end));
    assert_eq!(counter.to_string(), "00:00:10");
    counter.flip();
    assert_eq!(counter.to_string(), "00:00:10");
    counter.flip();
    assert_eq!(counter.to_string(), "00:00:10");
}
