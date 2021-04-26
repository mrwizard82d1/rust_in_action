fn main() {
    let search_term = "picture";
    let quote = "\
Every face, every shop, bedroom window, public-house, and
dark square is a picture feverishly turned--in search of what?
It is the same with books. What do we seek through millions of pages?";

    // Automatically increment line number using the `enumerate` method
    // Because the lines() method returns an iterator, it can be chained to invoke `enumerate`
    for (i, line) in quote.lines().enumerate() {
        if line.contains(search_term) {
            // Avoid calculating line number at each and every step
            let line_num = i + 1; // line numbers begin at 1 but enumeration begins at 0
            println!("{}: {}", line_num, line)
        }
    }
}
