use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

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

    let mut cont: i32 = 0;

    for line in reader.lines() {
        let mut left: i8 = -1;
        let mut right: i8 = -1;
        match line {
            Ok(content) => {
                for c in content.chars() {
                    if !c.is_digit(10) {
                        continue;
                    }

                    if left == -1 {
                        left = c.to_digit(10).unwrap() as i8;
                        right = c.to_digit(10).unwrap() as i8;
                    } else {
                        right = c.to_digit(10).unwrap() as i8;
                    }
                }
            }
            Err(e) => {
                eprintln!("An error has ocurred. {}", e);
                std::process::exit(1);
            }
        }

        cont += (left * 10 + right) as i32;
    }

    eprintln!("{}", cont);
}
