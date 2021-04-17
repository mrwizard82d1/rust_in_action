#[derive(Debug)] // Allows the `println!` macro to print the Cereal enum.
enum Cereal {
    Barley, Millet, Rice,
    Rye, Spelt, Wheat,
}

fn main() {
    // Initializes an empty array of `Vec<Cereal>`.
    // The `mut` keyword allows us to change items in the vector?
    let mut grains: Vec<Cereal> = vec![];
    grains.push(Cereal::Rye); // Add an item to the vector.
    drop(grains); // Drops the vector **and** each item.

    // This line prevents this code from **compiling**. (The compiler detects an attempt to
    // reference a value that has been deleted.
    println!("{:?}", grains);
}
