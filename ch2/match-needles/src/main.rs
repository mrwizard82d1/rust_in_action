fn main() {
    // let needle = 42; // the variable needle is now redundant
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        // The `match` expression returns a value - which can then be bound to a variable.
        let result = match item {
            42 | 132 => "hit!", // Eureka! Found a match.
            _ => "miss!", // The underscore is a "wildcard" that matches any value
        };

        if result == "hit!" {
            println!("{:3}: {}", item, result)
        }
    }
}
