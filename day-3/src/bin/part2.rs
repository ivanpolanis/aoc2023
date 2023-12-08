#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Number {
    num: u32,
    pos: Position,
    len: usize,
    is_checked: bool,
}

fn parse_input(input: String) -> Vec<Number> {
    let mut parsed: Vec<Number> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut num = String::new();
        let mut sx: isize = -1;
        for (x, char) in line.chars().enumerate() {
            if !char.is_digit(10) {
                if !num.is_empty() {
                    let parsed_num: u32 = num.parse().unwrap();
                    parsed.push(Number {
                        num: parsed_num,
                        pos: Position { x: sx as usize, y },
                        len: (x - sx as usize),
                        is_checked: false,
                    });
                    num = String::new();
                }
                sx = -1;
                continue;
            }
            if sx == -1 {
                sx = x as isize;
            }
            num.push(char);
        }
        if !num.is_empty() {
            let parsed_num: u32 = num.parse().unwrap();
            parsed.push(Number {
                num: parsed_num,
                pos: Position { x: sx as usize, y },
                len: (line.len() - sx as usize),
                is_checked: false,
            });
        }
    }
    parsed
}

fn main() {
    let input = include_str!("input.txt");

    let mut parsed = parse_input(input.to_string());

    let mut count = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char.is_digit(10) || char != '*' {
                continue;
            }

            let mut ratios: Vec<u32> = Vec::new();
            for dx in (x as isize - 1).max(0)..=(x + 1) as isize {
                for dy in (y as isize - 1).max(0)..=(y + 1) as isize {
                    for num in parsed.iter_mut() {
                        if dy as usize == num.pos.y
                            && dx as usize >= num.pos.x
                            && (dx as usize) < num.pos.x + num.len
                            && !num.is_checked
                        {
                            num.is_checked = true;
                            ratios.push(num.num);
                        }
                    }
                }
            }

            if ratios.len() == 2 {
                count += ratios.first().unwrap() * ratios.last().unwrap();
            }
        }
    }

    println!("{count}");
}
