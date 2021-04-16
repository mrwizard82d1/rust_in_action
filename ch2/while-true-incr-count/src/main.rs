// Brings in both `Duration` and `Instant` into local scope
use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0); // create a duration of 1 second
    let start = Instant::now(); // access the system clock

    // An Instant minus an Instant is a Duration
    while (Instant::now() - start) < time_limit {
        count += 1;
    }
    println!("{}", count);
}
