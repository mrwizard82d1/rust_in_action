fn main() {
    let needle = 0xCB; // 203 decimal
    let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    // All types with the `Iterator` trait implement a method named `iter`.
    // The "trick" of borrowing using a reference to the array **only** works with arrays.
    // Calling `iter` works with **all** types implementing the `Iterator` trait.
    for item in haystack.iter() {
        if *item == needle {
            println!("{}", item);
        }
    }
}
