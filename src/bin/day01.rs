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
    println!("Result is: {:?}", output);
}

fn process(input_path: &str) -> (usize, usize) {
    let content = fs::read_to_string(input_path)
        .expect("Could not read the input file");

    let depths = content.split("\n").collect::<Vec<&str>>()
        .iter().filter(|x| !x.is_empty())
        .collect::<Vec<&&str>>()
        .iter().map(|&x| x.parse::<u32>().expect("Wrong element in list"))
        .collect::<Vec<u32>>();

    (get_depth_increases(&depths), get_depth_increases_windows(&depths))
}

fn get_depth_increases(depths: &Vec<u32>) -> usize {
    /* First part of the puzzle: count how many times the depth increases */
    let mut increases = 0;
    let mut cur_depth: u32 = 0;

    for depth in depths.iter() {
        if (cur_depth != 0) && (*depth > cur_depth) {
            increases += 1;
        }

        cur_depth = *depth;
    }

    increases
}

fn get_depth_increases_windows(depths: &Vec<u32>) -> usize {
    /* Second part of the puzzle: compute the depths as "windows" of the sum
     * of 3 sequential depths and count the new increases */
    let mut increases = 0;
    let mut depths_windows: Vec<u32> = vec!();
    let mut idx = 0;
    while idx < (depths.len() - 2) {
        let window = depths.get(idx).expect("Wrong window index") +
                    depths.get(idx + 1).expect("Wrong window index") +
                    depths.get(idx + 2).expect("Wrong window index");

        depths_windows.push(window);
        idx += 1;
    }

    let mut cur_depth = 0;
    for depth in depths_windows.iter() {
        if (cur_depth != 0) && (*depth > cur_depth) {
            increases += 1;
        }

        cur_depth = *depth;
    }

    increases
}
