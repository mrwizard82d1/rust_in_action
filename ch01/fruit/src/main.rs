fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];

    // Attempt to access an item outside the vector bounds.
    let buffer_overflow = fruit[4];

    // Macro asserts that two expressions have equal values. Causes panic if false.
    // assert_eq!(buffer_overflow, '🍉') // <2>
    // Even though `assert_eq!` causes a panic on failure, simply accessing a value outside the
    // bounds of an array produces a panic without the assertion.
    println!("{}", buffer_overflow);
}
