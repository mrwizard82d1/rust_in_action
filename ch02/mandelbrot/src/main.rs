// Imports the `Complex` number type from the `num` crate and its `complex` submodule
use num::complex::Complex;


// Converts between output space (a grid of rows and columns) and a range that surrounds the
// Mandelbrot set (a continuous region near (0, 0)).
fn calculate_mandelbrot(
    // If a number has not "escaped" within `max_iters`, it is consider in the Mandelbrot set
    max_iters: usize,
    x_min: f64, // Parameters specifying
    x_max: f64, // the search space within
    y_min: f64, // which we seek members
    y_max: f64, // of the Mandelbrot set
    width: usize, // The size of the output in pixels
    height: usize) -> Vec<Vec<usize>> {
    // Create a container for the data from each row
    let mut all_rows: Vec<Vec<usize>> = Vec::with_capacity(width);
    for img_y in 0..height { // iterate row-by-row allowing printing line-by-line
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            // Calculate the proportion of space covered in our output and converts that proportion
            // to values within our search space.
            let cx = x_min + (x_max - x_min) * (img_x as f64 / width as f64);
            let cy = y_min + (y_max - y_min) * (img_y as f64  / height as f64);
            // The values `cx` and `cy` represent the real and imaginary components o complex
            // number used in calculating members of the Mandelbrot set.
            let escaped_at = mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }
        all_rows.push(row);
    }
    all_rows
}

// The function called at every pixel (e.g., for every row ond column that's printed to `stdout`)
fn mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    let mut z = Complex {re: 0.0, im: 0.0}; // initialize a complex number at the origin
    let c = Complex::new(cx, cy); // initializes a second complex number

    for i in 0..=max_iters {
        if z.norm() > 2.0 { // checks the escape condition returning the iteration if escaped
            return i;
        }
        z = z * z + c; // repeatedly mutates z to check if c lies within the Mandelbrot set
    }
    return max_iters; // remember that `i` is **no longer** in scope
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => 'o',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            line.push(val);
        }
        println!("{}", line);
    }
}


fn main() {
    let mandelbrot = calculate_mandelbrot(1000, -2.0, 1.0, -1.0, 1.0, 100, 24);

    render_mandelbrot(mandelbrot);
}
