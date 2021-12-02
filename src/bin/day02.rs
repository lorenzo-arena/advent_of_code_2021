use clap;
use std::fs;

fn main() {
    let matches = clap::App::new("day02")
        .version("0.0.1")
        .author("Lorenzo A.")
        .about("Day 02 implementation for Advent Of Code 2021")
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
    println!("Result is: {:?} which multiplied gives {}", output, output.0 * output.1);

    let output_aim = process_with_aim(input_path);
    println!("Result with aim is: {:?} which multiplied gives {}", output_aim, output_aim.0 * output_aim.1);
}

fn process(input_path: &str) -> (usize, usize) {
    let content = fs::read_to_string(input_path)
        .expect("Could not read the input file");

    let instructions = content.split("\n").collect::<Vec<&str>>()
        .into_iter().filter(|instr| !instr.is_empty()).collect::<Vec<&str>>();

    calculate_dest(&instructions)
}

fn process_with_aim(input_path: &str) -> (usize, usize) {
    let content = fs::read_to_string(input_path)
        .expect("Could not read the input file");

    let instructions = content.split("\n").collect::<Vec<&str>>()
        .into_iter().filter(|instr| !instr.is_empty()).collect::<Vec<&str>>();

    calculate_dest_with_aim(&instructions)
}

fn calculate_dest(instructions: &Vec<&str>) -> (usize, usize) {
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;

    for instr in instructions {
        let split_instr = instr.split(" ").collect::<Vec<&str>>();
        let dir = split_instr.get(0).unwrap();
        let size = split_instr.get(1).unwrap().parse::<usize>().unwrap();
        match *dir {
            "forward" => horizontal += size,
            "down" => depth += size,
            "up" => depth -= size,
            _ => panic!("Unknown instruction found")
        }
    }

    (horizontal, depth)
}

fn calculate_dest_with_aim(instructions: &Vec<&str>) -> (usize, usize) {
    let mut horizontal: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;

    for instr in instructions {
        let split_instr = instr.split(" ").collect::<Vec<&str>>();
        let dir = split_instr.get(0).unwrap();
        let size = split_instr.get(1).unwrap().parse::<usize>().unwrap();
        match *dir {
            "forward" => {
                horizontal += size;
                depth += aim * size;
            },
            "down" => aim += size,
            "up" => aim -= size,
            _ => panic!("Unknown instruction found")
        }
    }

    (horizontal, depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_dest() {
        let mut instructions: Vec<&str> = vec!();
        instructions.push("forward 5");
        instructions.push("down 5");
        instructions.push("forward 8");
        instructions.push("up 3");
        instructions.push("down 8");
        instructions.push("forward 2");

        let res = calculate_dest(&instructions);

        assert_eq!(res.0, 15);
        assert_eq!(res.1, 10);
    }

    #[test]
    fn test_calculate_dest_with_aim() {
        let mut instructions: Vec<&str> = vec!();
        instructions.push("forward 5");
        instructions.push("down 5");
        instructions.push("forward 8");
        instructions.push("up 3");
        instructions.push("down 8");
        instructions.push("forward 2");

        let res = calculate_dest_with_aim(&instructions);

        assert_eq!(res.0, 15);
        assert_eq!(res.1, 60);
    }
}
