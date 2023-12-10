use std::collections::HashMap;

struct Hand {
    cards: Vec<u32>,
}

fn parse_input(input: &str) {
    let card_values: HashMap<&str, u16> = HashMap::from([
        ("A", 0xD),
        ("K", 0xC),
        ("Q", 0xB),
        ("J", 0xA),
        ("T", 0x9),
        ("9", 0x8),
        ("8", 0x7),
        ("7", 0x6),
        ("6", 0x5),
        ("5", 0x4),
        ("4", 0x3),
        ("3", 0x2),
        ("2", 0x1),
    ]);

    for line in input.lines() {
        let handed = line.split_ascii_whitespace().nth(0).unwrap();

        let handed: Vec<u16> = match handed.chars().map(|x| card_values.get(x)).collect() {
            Ok(n) => n,
            Err(_) => std::process::exit(1),
        };
    }
}

fn main() {
    let input = include_str!("sample.txt");
}
