fn main() {
    let twenty = 20; // Rust infers the variable type if not supplied
    let twenty_one: i32 = 21; // The developer can specify a concrete type
    let twenty_two = 22i32; // Or include the type in the integer literal

    let sum = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, sum);

    // Underscores increase readability for humans but are ignored by the compiler
    let one_million: i64 = 1_000_000;
    println!("{}", one_million.pow(2)); // Remember that numbers have methods

    // Array items must all have the same type
    let forty_twos = [
        42.0, // Floating point literal (defaults to single precision)
        42f32, // Floating point literals can also have type suffixes
        42.0_f32, // And underscores
    ];
    // Elements of arrays indexed numerically beginning with 0
    // Format `:02` indicates format the value in a field of width 2 with a leading zero for
    // padding (if needed)
    println!("{:02}", forty_twos[0]);
}
