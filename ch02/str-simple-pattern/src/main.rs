fn main() {
    let search_term = "picture";
    // Multi-line strings require **no** special syntax. The `\` character in line 4 escapes
    // the initial newline character in the string.
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    // The `lines()` method returns an iterator over all the lines in the string. The definition
    // of a line depends on the operating system's convention on what constitutes a new line.
    for line in quote.lines() {
        // The `contains` method searches for one string within another.
        if line.contains(search_term) {
            println!("{}", line)
        }
    }
}
