use clap;
use std::fs;

fn main() {
    let matches = clap::App::new("day01")
        .version("0.0.1")
        .author("Lorenzo A.")
        .about("Day 01 implementation for Advent Of Code 2021")
        .arg(
            clap::Arg::with_name("input")
                .long("input")
                .value_name("FILE")
                .help("Path to the problem input")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let input_path = matches.value_of("input").unwrap();
    let output = process(input_path);
    println!("Result is: {}", output);
}

fn process(input_path: &str) -> usize {
    let content = fs::read_to_string(input_path)
        .expect("Could not read the input file");

    0
}
