use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Game {
    id: u32,
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    fn new(id: u32) -> Game {
        return Game {
            id,
            red: 0,
            blue: 0,
            green: 0,
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("{} <filename>", args[0]);
        std::process::exit(1);
    }

    let file_name = &args[1];

    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("An error has ocurred. {}", e);
            std::process::exit(1);
        }
    };

    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let mut actual_game = Game::new();
    }
}
