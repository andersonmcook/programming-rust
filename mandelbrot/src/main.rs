use std::{env, process};

mod parser;
mod renderer;
mod writer;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.0.20",
            args[0]
        );
        process::exit(1);
    }

    let bounds = parser::dimensions(&args[2]).expect("error parsing image dimensions");
    let upper_left = parser::complex(&args[3]).expect("error parsing upper left corner point");
    let lower_right = parser::complex(&args[4]).expect("error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    renderer::parallel_render(&mut pixels, bounds, upper_left, lower_right);
    writer::write(&args[1], &pixels, bounds).expect("error writing PNG file");
}
