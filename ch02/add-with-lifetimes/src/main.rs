// Lifetime parameters are a way of providing control to the programmer while maintaining
// high-level code.

// noinspection RsNeedlessLifetimes
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    // Adds the values referred to by i and j instead of the values themselves
    *i + *j
}

fn main() {
    let a = 10;
    let b = 20;

    // Do not require lifetime notation on a call, but since references are expected, we must use
    // the `&` operator.
    let sum = add_with_lifetimes(&a, &b);

    println!("{}", sum);
}
