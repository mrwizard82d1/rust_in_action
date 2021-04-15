fn main() {
    let three = 0b11; // Initialize integer using binary literal
    let thirty = 0o36; // octal literal
    let three_hundred = 0x12C; // and hexadecimal literal

    println!("base 10: {:9} {:9} {:9}", three, thirty, three_hundred);
    // Output as binary
    println!("base  2: {:9b} {:9b} {:9b}", three, thirty, three_hundred);
    // Output as octal
    println!("base  8: {:9o} {:9o} {:9o}", three, thirty, three_hundred);
    // Output as hexadecimal (small letters)
    println!("base 16: {:9x} {:9x} {:9x}", three, thirty, three_hundred);
}
