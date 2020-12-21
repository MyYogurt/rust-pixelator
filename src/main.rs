mod pixelate;

extern crate clap;

use clap::{App, Arg};

fn main() {
    let matches = App::new("rust-pixelator")
        .arg(Arg::with_name("input")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("Input file path"))
        .arg(Arg::with_name("output")
            .required(true)
            .takes_value(true)
            .index(2)
            .help("Output file path")
        ).get_matches();
    let path = matches.value_of("input").unwrap();
    let output_path = matches.value_of("output").unwrap();
    pixelate::pixelate_image(path, output_path, 12);
}
