use std::collections::HashSet;

fn main() -> Result<(), ()> {
    let input = include_str!("input.txt");

    let mut count = 0;

    for line in input.lines() {
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

        if wins > 0 {
            count += isize::pow(2, (wins - 1) as u32);
        }
    }

    println!("{count}");

    Ok(())
}
