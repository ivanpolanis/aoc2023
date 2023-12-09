use std::num::ParseIntError;

fn parse_input(input: &str) -> String {
    let res: String = input
        .split(": ")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect();

    return res;
}

fn main() -> Result<(), ParseIntError> {
    let input = include_str!("input.txt");

    let time: u128 = parse_input(input.lines().nth(0).unwrap()).parse().unwrap();
    let distance: u128 = parse_input(input.lines().nth(1).unwrap()).parse().unwrap();

    let mut count = 1;

    for j in 1..time {
        let distance_traveled: u128 = (time - j) * j;

        if distance_traveled > distance {
            count *= time - 2 * j + 1;
            break;
        }
    }

    println!("{count}");

    Ok(())
}
