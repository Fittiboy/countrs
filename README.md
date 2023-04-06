# About
This library provides a simple method for counting down to (or up from) a point in time that can be shifted back and forth.  
# Examples

A counter stores `start` and `end` times, and implements `Display`
to either show the time passed since `start`, or until `end`,
formatted as `HH(+):MM:SS`.  
The timer will not go down past 00:00:00.
# Examples
Basic functionality is very simple:
```rust
let now = TimeStamp::now();
let mut counter = Counter::down(
    Some(now - 600),
    Some(now + 600)
);

// A small amount of time will have passed since `now` was assigned
assert_eq!(counter.to_string(), "00:09:59");
counter.flip();
// It now counts up from `start`
assert_eq!(counter.to_string(), "00:10:00")
```
Both `start` and `end` times are adjustable:
```rust
let mut counter = Counter::up(Some(TimeStamp::now()), None);
counter.try_move_start(-30).unwrap();

assert_eq!(counter.to_string(), "00:00:30")
```


# Documentation
The full documentation is available [here on docs.rs](https://docs.rs/countrs)!

# Motivation
The primary motivation for the creation of this library is the concept of a marathon livestream, where the stream begins with a timer set to e.g. 4 hours at the beginning, and viewers can increase this time by paying.
