fn main() {
    let a = 10; // An (default size) integer
    let b: i32 = 20; // A 32-bit integer explicitly declared
    let c = 30i32; // A type annotation in an integer literal
    let d= 30_i32; // Numbers can include underscores for readability
    let e = add(add(a, b), add(c, d));
    println!("(a + b) + (c + d) = {}", e);
}

// Type declarations are **required** when defining functions
fn add(i: i32, j: i32) -> i32 {
    // Functions return the last expression evaluated in their scope
    // i + j; However, a trailing semicolon actually **suppresses** the value of the expression
    // so expression to be returned cannot have a trailing semicolon (unless the function returns
    // unit `()`).
    // (I suspect, like F#, Rust does not allow one to ignore values returned by an expression
    // which is why semicolons are "required" on most statements. Although the error message
    // when I tried this simply said the semicolon was required. (puzzled))
    i + j
}
