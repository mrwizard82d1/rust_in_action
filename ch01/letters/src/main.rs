fn main() {
    // Binds a mutable value to `letters`.
    let mut letters = vec! {
        "a", "b", "b",
    };

    for letter in letters {
        println!("{}", letter);
        // Attempt to mutate `letters` - but we are in the midst of **iterating over letters**
        // Fails at **compile-time**.
        letters.push(letter.clone());
    }
}
