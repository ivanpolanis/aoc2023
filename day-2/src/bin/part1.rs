struct Game {
    id: usize,
    red: u32,
    blue: u32,
    green: u32,
}

impl Game {
    fn new(id: usize) -> Game {
        return Game {
            id,
            red: 0,
            blue: 0,
            green: 0,
        };
    }

    fn is_valid(self) -> bool {
        return self.red <= 12 && self.green <= 13 && self.blue <= 14;
    }
}

fn main() -> Result<(), std::num::ParseIntError> {
    let file = include_str!("input.txt");

    let mut count = 0;

    for (i, line) in file.lines().enumerate() {
        let mut card = Game::new(i + 1);
        let game = line.split(": ").last().unwrap();
        let rounds = game.split("; ");
        for round in rounds {
            let data = round.split(", ");
            for j in data {
                let mut splitted = j.split_ascii_whitespace();
                let qty: u32 = match splitted.next().unwrap().parse() {
                    Ok(n) => n,
                    Err(e) => std::process::exit(1),
                };
                let color = splitted.next().unwrap();

                match color {
                    "red" => {
                        if card.red < qty {
                            card.red = qty
                        }
                    }
                    "blue" => {
                        if card.blue < qty {
                            card.blue = qty
                        }
                    }
                    "green" => {
                        if card.green < qty {
                            card.green = qty
                        }
                    }
                    _ => std::process::exit(1),
                }
            }
        }

        if card.is_valid() {
            count += i + 1;
        }
    }

    println!("{count}");

    Ok(())
}
