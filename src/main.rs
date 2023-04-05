use countrs::times::{Duration, TimeStamp};
use countrs::Counter;

fn main() {
    let down = Counter::down(
        Some(TimeStamp::now()),
        Some(TimeStamp::now() + Duration::days(20)),
    );
    let up = Counter::up(
        Some(TimeStamp::now() - Duration::days(20)),
        Some(TimeStamp::now()),
    );
    println!("Down: {}\nUp: {}", down, up)
}
