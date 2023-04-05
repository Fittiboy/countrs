use chrono::{Duration, Utc};
use countrs::Counter;

fn main() {
    let down = Counter::down(Some(Utc::now()), Some(Utc::now() + Duration::days(20)));
    let up = Counter::up(Some(Utc::now() - Duration::days(20)), Some(Utc::now()));
    println!("Down: {}\nUp: {}", down, up)
}
