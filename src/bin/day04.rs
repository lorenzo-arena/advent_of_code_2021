use clap;
use std::fs;
use regex::Regex;


fn main() {
    let matches = clap::App::new("day04")
        .version("0.0.1")
        .author("Lorenzo A.")
        .about("Day 04 implementation for Advent Of Code 2021")
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

fn process(input_path: &str) -> (u32, u32) {
    let content = fs::read_to_string(input_path)
        .expect("Could not read the input file");

    let entries = content.split("\n\n").collect::<Vec<&str>>()
        .into_iter().filter(|entry| !entry.is_empty()).collect::<Vec<&str>>();

    if entries.len() < 2 {
        panic!("Not enough elements for a game");
    }

    let to_extract = entries[0].split(",").collect::<Vec<&str>>()
        .into_iter().map(|x| x.parse::<u32>().expect("Wrong element in list"));

    let mut game = BingoGame::new();

    for board in &entries[1..entries.len()] {
        let mut nums: [Entry; 25] = [Entry::new(0); 25];
        let rows = board.split("\n").collect::<Vec<&str>>();
        let re = Regex::new(r"^[ ]{0,}([0-9]+)[ ]+([0-9]+)[ ]+([0-9]+)[ ]+([0-9]+)[ ]+([0-9]+)$").unwrap();

        let mut index = 0;
        for row in rows {
            if !row.is_empty() {
                let caps = match re.captures(row) {
                    None => panic!("Can't parse board content"),
                    Some(captures) => captures
                };

                nums[index].num = caps.get(1).map(|m| m.as_str()).unwrap().parse::<u32>().unwrap();
                nums[index + 1].num = caps.get(2).map(|m| m.as_str()).unwrap().parse::<u32>().unwrap();
                nums[index + 2].num = caps.get(3).map(|m| m.as_str()).unwrap().parse::<u32>().unwrap();
                nums[index + 3].num = caps.get(4).map(|m| m.as_str()).unwrap().parse::<u32>().unwrap();
                nums[index + 4].num = caps.get(5).map(|m| m.as_str()).unwrap().parse::<u32>().unwrap();
            }

            index += 5;
        }

        game.add_board(Board::new(nums));
    }

    let mut index = 0;
    let mut winner_points: Option<u32> = None;

    for num in to_extract {
        game.extract(num, index);
        index += 1;

        if game.game_ended() && winner_points.is_none() {
            winner_points = Some(game.get_winner_points());
        }
    }

    (winner_points.unwrap(), game.get_last_winner_points())
}

#[derive(Copy, Clone)]
struct Entry {
    num: u32,
    is_matched: bool
}

impl Entry {
    fn new(num: u32) -> Entry {
        Entry {
            num,
            is_matched: false
        }
    }

    fn set_is_matched(&mut self, matched: bool) {
        self.is_matched = matched;
    }

    fn is_matched(&self) -> bool {
        self.is_matched
    }
}

const ROWS: usize = 5;
const COLUMNS: usize = 5;

#[derive(Clone)]
struct Board {
    numbers: [Entry; COLUMNS * ROWS],
    winning_num: Option<u32>,
    winning_index: Option<u32>
}

impl Board {
    fn new(numbers: [Entry; COLUMNS * ROWS]) -> Board {
        Board {
            numbers,
            winning_num: None,
            winning_index: None
        }
    }

    fn extract(&mut self, num: u32, index: u32) {
        if !self.check_win() {
            let mut i = 0;
    
            while i < (COLUMNS * ROWS) {
                if self.numbers[i].num == num {
                    self.numbers[i].set_is_matched(true);
                }
    
                i += 1;
            }
    
            if self.check_win() && self.winning_num.is_none() && self.winning_index.is_none() {
                self.winning_num = Some(num);
                self.winning_index = Some(index);
            }
        }
    }

    fn check_rows(&self) -> bool {
        let mut x = 0;

        while x < ROWS {
            let mut y = 0;
            let mut win = true;

            while y < COLUMNS {
                win &= self.numbers[x * COLUMNS + y].is_matched();
                y += 1;
            }

            if win {
                return true;
            }

            x += 1;
        }

        false
    }

    fn check_columns(&self) -> bool {
        let mut y = 0;

        while y < COLUMNS {
            let mut x = 0;
            let mut win = true;

            while x < ROWS {
                win &= self.numbers[y + (x * COLUMNS)].is_matched();
                x += 1;
            }

            if win {
                return true;
            }

            y += 1;
        }

        false
    }

    fn check_win(&self) -> bool {
        self.check_rows() || self.check_columns()
    }

    /* The total points made by a winning board is made of the sum of all unmatched numbers multiplied
     * by the number called when the board won */
    fn get_points(&self) -> u32 {
        let mut i = 0;
        let mut points = 0;

        while i < (COLUMNS * ROWS) {
            if !self.numbers[i].is_matched() {
                points += self.numbers[i].num;
            }

            i += 1;
        }

        points * self.winning_num.unwrap()
    }
}

struct BingoGame {
    boards: Vec<Board>
}

impl BingoGame {
    fn new() -> BingoGame {
        BingoGame {
            boards: vec!()
        }
    }

    fn add_board(&mut self, board: Board) {
        self.boards.push(board);
    }

    fn extract(&mut self, num: u32, index: u32) {
        for board in &mut self.boards {
            board.extract(num, index);
        }
    }

    fn game_ended(&self) -> bool {
        self.boards.iter().any(|board| board.check_win())
    }

    fn get_winner_points(&self) -> u32 {
        match self.boards.iter().find(|board| board.check_win()) {
            None => panic!("No board has won the game yet"),
            Some(b) => b.get_points()
        }
    }

    fn get_last_winner_points(&self) -> u32 {
        let last_winner = self.boards.iter().filter(|board| board.check_win())
            .max_by(|a, b| a.winning_index.unwrap().cmp(&b.winning_index.unwrap()));

        last_winner.unwrap().get_points()
    }
}
