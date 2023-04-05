use crate::chrono::*;
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
