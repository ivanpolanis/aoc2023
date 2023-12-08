use std::collections::HashSet;

fn main() -> Result<(), ()> {
    let input = include_str!("input.txt");

    let mut count = 0;
    let mut cards_won: Vec<usize> = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let ours = line.split(": ").last().unwrap().split("| ").last().unwrap();
        let winners = line.split(": ").last().unwrap().split("| ").nth(0).unwrap();

        let ours: HashSet<u32> = match ours
            .split_ascii_whitespace()
            .into_iter()
            .map(|x| x.parse())
            .collect()
        {
            Ok(n) => n,
            Err(_e) => std::process::exit(1),
        };

        let winners: HashSet<u32> = match winners
            .split_ascii_whitespace()
            .into_iter()
            .map(|x| x.parse())
            .collect()
        {
            Ok(n) => n,
            Err(_e) => std::process::exit(1),
        };

        let intersect = ours.intersection(&winners);

        let wins = intersect.count();

        for j in i + 1..=i + wins {
            cards_won[j] += cards_won[i];
        }
    }

    cards_won.iter().for_each(|x| count += x);

    println!("{count}");

    Ok(())
}
