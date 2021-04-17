// The fragment `<T: std::ops::Add<Output = t>>` requires that T must implement `Add` (support
// the `Add` trait?) and that the result must be of the same type as the input.
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let sum = add(10, 20);

    println!("{}", sum);
}
