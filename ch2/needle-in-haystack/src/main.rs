fn main() {
    let needle = 0o52; // 42 decimal
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    // Iterates over each element in `haystack`. Each iteration moves to the next element.
    for item in &haystack {
        if *item == needle { // Dereferences the item referred to during the iteration
            println!("{}", item);
        }
    }
}
