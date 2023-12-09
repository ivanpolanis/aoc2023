use std::num::ParseIntError;

fn parse_input(input: &str) -> Vec<u32> {
    let res: Vec<u32> = match input
        .split(": ")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse())
        .collect()
    {
        Ok(n) => n,
        Err(_e) => std::process::exit(1),
    };

    return res;
}

fn main() -> Result<(), ParseIntError> {
    let input = include_str!("input.txt");

    let time_list: Vec<u32> = parse_input(input.lines().nth(0).unwrap());
    let distance_list: Vec<u32> = parse_input(input.lines().nth(1).unwrap());

    let mut count = 1;

    for i in 0..time_list.len() {
        for j in 1..time_list[i] {
            let distance: u32 = (time_list[i] - j) * j;

            if distance > distance_list[i] {
                count *= time_list[i] - 2 * j + 1;
                break;
            }
        }
    }

    println!("{count}");

    Ok(())
}
