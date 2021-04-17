fn main() {
    let needle = 0o204; // 132 decimal
    // The book has a footnote for the following line stating, "Dereferencing item is not
    // required as it's already returned by value." However, when compiling this code, the
    // compiler reports an error as follows:
    // 15 |         if item == needle {
    //    |                 ^^ no implementation for `&{integer} == {integer}`
    //
    // Based on this
    // [Stack Overflow Post](https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter),
    // I discovered that **Arrays** implement `into_iter` "(in such a surprising fashion) to make
    // it possible to iterate over references to them in `for` loops."
    //
    // When I checked the book, I discovered that it actually did not declare `haystack` to be an
    // `Array` but a `Vector`!
    // let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796];
    let haystack = vec![1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862, 16796];

    // A call to `into_iter()` iterates over its collection but returns values of the collection
    // **by value** instead of by reference.
    // The description of returning items by value from the book, "Rust in Action," makes this
    // statement, "As a consequence, this (returning by value) prevents other parts of the code
    // from accessing those values." I do not fully understand this sentence. Perhaps after
    // chapter 4 (which, I believe talks about "borrowing" and "ownership").
    for item in haystack.into_iter() {
        if item == needle {
            println!("{}", item);
        }
    }
}
