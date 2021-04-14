fn main() {
    let penguins_data = "\
    common name,length (cm)
    Little penguins,33
    Yellow-eyed penguins,65
    Fiordland penguins,70
    Invalid,data
    ";

    let records = penguins_data.lines();

    for (i, record) in records.enumerate() {
        // Skip header and empty records
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        // `Vec` is shorthand for `Vector`
        let fields: Vec<_> = record
            .split(',')  // splits record into substrings
            // "for loops" accept higher-order functions.
            // higher-order function trims whitespace from either end of substrings
            .map(|field| field.trim())
            .collect();  // "Collects" results of an iterator into a `Vector`

        // This code block only runs under a debug configuration.
        // Remember that `!` indicates a macro and not a function.
        if cfg!(debug_assertions) {
            // following line should not exceed **55** characters; please add line break and move
            // annotation marker as needed.
            // Prints to `stderr`. The `{:?}` syntax requests the default debugging representation
            // for the two types as output.
            eprintln!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0]; // Collections indexed by integer

        // following line should not exceed **55** characters; please add line break and move
        // annotation marker as needed.
        // parses a string into another using the type information on the **left-hand side**
        // of the assignment operator. Note that `parse` either returns a value or an error
        // value. The returned value is wrapped in a `Result`. The `_` requests the compiler
        // to infer the **error*** type itself.
        let maybe_length: Result<f32, _> = fields[1].parse();
        // Skip record with an invalid length
        if maybe_length.is_err() {
            continue;
        }

        let length = maybe_length.unwrap(); // unwraps the `f32` value from the `Result`
        // Prints to `stdout`. The `{}` syntax indicates that Rust should use a programmer-defined
        // method to represent the value as a string instead of its debug representation available
        // with `{:?}`.
        println!("{}, {}cm", name, length);
    }
}
