mod pixelate;

extern crate clap;
extern crate imagesize;

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
            .help("Output file path"))
        .arg(Arg::with_name("block count")
            .required(false)
            .takes_value(true)
            .short("b")
            .long("block")
            .help("Use to specify block count. Otherwise, a block count will be chosen for you"))
        .arg(Arg::with_name("list")
            .short("l")
            .long("list")
            .help("Use this flag to find all valid block counts"))
        .get_matches();

    let path: &str = matches.value_of("input").unwrap();
    let output_path: &str = matches.value_of("output").unwrap();

    if matches.is_present("list") {
        let (width, height) = match imagesize::size(path) {
            Ok(dim) => (dim.width, dim.height),
            Err(_) => {
                println!("Error reading file, aborting...");
                std::process::exit(1);
            }
        };
        let results: Vec<usize> = pixelate::find_possible_block_counts(width, height);
        println!("Valid block counts:");
        for block in results {
            println!("{}", block);
        }
        std::process::exit(0);
    }



    if let Some(num) = matches.value_of("block count") {
        let number = num.parse::<usize>();
        match number {
            Ok(res) => {
                pixelate::pixelate_image(path, output_path, res);
                std::process::exit(0);
            },
            Err(_) => println!("Invalid block count argument, choosing block count automatically"),
        }
    }

    let (width, height) = match imagesize::size(path) {
        Ok(dim) => (dim.width, dim.height),
        Err(_) => {
            println!("Error reading file, aborting...");
            std::process::exit(1);
        }
    };

    let vec: Vec<usize> = pixelate::find_possible_block_counts(width, height);
    let block_size = vec[vec.len()/4];
    pixelate::pixelate_image(path, output_path, block_size);
}
