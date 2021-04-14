fn main() {
    let fruit = vec!['🥝', '🍌', '🍇'];

    // Attempt to access an item outside the vector bounds.
    let buffer_overflow = fruit[4];

    // Macro asserts that two expressions have equal values. Causes panic if false.
    assert_eq!(buffer_overflow, '🍉') // <2>
}
