use std::thread;


fn main() {
    // `data` is a variable initialized to 100
    let mut data = 100;

    // Spawn a thread that assigns 500 to the "variable" `data`
    thread::spawn(|| { data = 500; });
    // Also spawn a thread that assigns 1000 to the "variable" `data`
    thread::spawn(|| { data = 1000; });

    // The problem: what is the deterministic value of `data`.
    // In this ambiguous situation, Rust will again fail to compile this code.
    println!("{}", data)
}
