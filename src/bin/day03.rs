use clap;
use std::fs;

fn main() {
    let matches = clap::App::new("day03")
        .version("0.0.1")
        .author("Lorenzo A.")
        .about("Day 03 implementation for Advent Of Code 2021")
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
    println!("gamma rate is {}, epsilon rate is {}; multiplied value is {}", output.0, output.1, output.0 * output.1);

    let output = process_life_supporting(input_path);
    println!("oxygen is {}, CO2 scrubber is {}; multiplied value is {}", output.0, output.1, output.0 * output.1);
}

fn process(input_path: &str) -> (u32, u32) {
    let content = fs::read_to_string(input_path)
        .expect("Could not read the input file");

    let diagnostic = content.split("\n").collect::<Vec<&str>>()
        .into_iter().filter(|instr| !instr.is_empty()).collect::<Vec<&str>>();

    get_gamma_epsilon(&diagnostic)
}

fn process_life_supporting(input_path: &str) -> (u32, u32) {
    let content = fs::read_to_string(input_path)
        .expect("Could not read the input file");

    let diagnostic = content.split("\n").collect::<Vec<&str>>()
        .into_iter().filter(|instr| !instr.is_empty()).collect::<Vec<&str>>();

    get_life_supporting(&diagnostic)
}

fn get_gamma_epsilon(diagnostic: &Vec<&str>) -> (u32, u32) {
    let mut idx = 0;
    let mut go_on = true;
    let mut gamma_string = String::new();
    let mut epsilon_string = String::new();

    while go_on {
        let mut ones = 0;
        for entry in diagnostic {
            match entry.chars().nth(idx) {
                None => go_on = false,
                Some(c) => match c {
                    '1' => ones += 1,
                    '0' => (),
                    '\n' => go_on = false,
                    _ => panic!("Unknown char found")
                }
            }
        }

        if go_on {
            if ones > (diagnostic.len() / 2) {
                gamma_string += "1";
                epsilon_string += "0";
            } else {
                gamma_string += "0";
                epsilon_string += "1";
            }

            idx += 1;
        }
    }

    let gamma = u32::from_str_radix(&gamma_string[..], 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_string[..], 2).unwrap();

    (gamma, epsilon)
}

fn get_oxygen(diagnostic: &Vec<&str>, start_bit: usize) -> u32 {
    let mut ones: f64 = 0.0;
    for entry in diagnostic {
        match entry.chars().nth(start_bit).unwrap() {
            '1' => ones += 1.0,
            '0' => (),
            _ => panic!("Unknown char found")
        }
    }

    if ones >= (diagnostic.len() as f64 / 2.0) {
        let mut diag_subset: Vec<&str> = vec!();

        for entry in diagnostic {
            if entry.chars().nth(start_bit).unwrap() == '1' {
                diag_subset.push(entry);
            }
        }

        if diag_subset.len() == 1 {
            return u32::from_str_radix(diag_subset[0], 2).unwrap();
        } else {
            return get_oxygen(&diag_subset, start_bit + 1);
        }
    } else {
        let mut diag_subset: Vec<&str> = vec!();

        for entry in diagnostic {
            if entry.chars().nth(start_bit).unwrap() == '0' {
                diag_subset.push(entry);
            }
        }

        if diag_subset.len() == 1 {
            return u32::from_str_radix(diag_subset[0], 2).unwrap();
        } else {
            return get_oxygen(&diag_subset, start_bit + 1);
        }
    }
}

fn get_co2_scrubber(diagnostic: &Vec<&str>, start_bit: usize) -> u32 {
    let mut ones: f64 = 0.0;
    for entry in diagnostic {
        match entry.chars().nth(start_bit).unwrap() {
            '1' => ones += 1.0,
            '0' => (),
            _ => panic!("Unknown char found")
        }
    }

    if ones < (diagnostic.len() as f64 / 2.0) {
        let mut diag_subset: Vec<&str> = vec!();

        for entry in diagnostic {
            if entry.chars().nth(start_bit).unwrap() == '1' {
                diag_subset.push(entry);
            }
        }

        if diag_subset.len() == 1 {
            return u32::from_str_radix(diag_subset[0], 2).unwrap();
        } else {
            return get_co2_scrubber(&diag_subset, start_bit + 1);
        }
    } else {
        let mut diag_subset: Vec<&str> = vec!();

        for entry in diagnostic {
            if entry.chars().nth(start_bit).unwrap() == '0' {
                diag_subset.push(entry);
            }
        }

        if diag_subset.len() == 1 {
            return u32::from_str_radix(diag_subset[0], 2).unwrap();
        } else {
            return get_co2_scrubber(&diag_subset, start_bit + 1);
        }
    }
}


fn get_life_supporting(diagnostic: &Vec<&str>) -> (u32, u32) {
    (get_oxygen(diagnostic, 0), get_co2_scrubber(diagnostic, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_gamma_epsilon() {
        let mut diagnostic: Vec<&str> = vec!();
        diagnostic.push("00100");
        diagnostic.push("11110");
        diagnostic.push("10110");
        diagnostic.push("10111");
        diagnostic.push("10101");
        diagnostic.push("01111");
        diagnostic.push("00111");
        diagnostic.push("11100");
        diagnostic.push("10000");
        diagnostic.push("11001");
        diagnostic.push("00010");
        diagnostic.push("01010");

        let res = get_gamma_epsilon(&diagnostic);

        assert_eq!(res.0, 22);
        assert_eq!(res.1, 9);
    }

    #[test]
    fn test_get_life_supporting() {
        let mut diagnostic: Vec<&str> = vec!();
        diagnostic.push("00100");
        diagnostic.push("11110");
        diagnostic.push("10110");
        diagnostic.push("10111");
        diagnostic.push("10101");
        diagnostic.push("01111");
        diagnostic.push("00111");
        diagnostic.push("11100");
        diagnostic.push("10000");
        diagnostic.push("11001");
        diagnostic.push("00010");
        diagnostic.push("01010");

        let res = get_life_supporting(&diagnostic);

        assert_eq!(res.0, 23);
        assert_eq!(res.1, 10);
    }
}
