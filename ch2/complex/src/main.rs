// Pull the `num` crate into local scope. The `::` operator restricts what names are included in the
// local scope. Specifically, we only include the `Complex` type.
use num::complex::Complex;


fn main() {
    // Types do not **require** a constructor. One can initialize an instance of a type by using
    // its name (`Complex`) followed by initializing all its members inside a block.
    let a = Complex { re: 2.1, im: -1.2 };

    // Many types also implement a `new` method for convenience; however, this method **is not**
    // required by Rust.
    let b = Complex::new(11.1, 22.2);

    let sum = a + b;

    // The type, `Complex`, has two fields, `re` and `im`. One can access the value of these fields
    // by using the `.` operator followed by the field name.
    println!("{} + {}i", sum.re, sum.im);
}
