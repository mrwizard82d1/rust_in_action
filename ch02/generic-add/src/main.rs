// Although the book adds the `Add` trait to the local scope, the compiler, version 1.51,0, reports
// that the import is unused. Commenting this line out runs with no warnings.
// use std::ops::Add; // `Add` trait from `std::ops` to local scope
use std::time::{Duration}; // `Duration` class added from `std::time` to local scope

// The fragment `<T: std::ops::Add<Output = t>>` requires that T must implement `Add` (support
// the `Add` trait?) and that the result must be of the same type as the input.
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let float_sum = add(1.2, 3.4); // add with floating point values
    let int_sum = add(10, 20); // add integer values
    let duration_sum = add(Duration::new(5, 0), Duration::new(10, 0)); // add durations

    println!("{}", float_sum);
    println!("{}", int_sum);
    // Because `std::time::Duration` does not implement the `std::fmt::Display` trait, we fallback
    // to `std::fmt::Debug`
    println!("{:?}", duration_sum);
}
