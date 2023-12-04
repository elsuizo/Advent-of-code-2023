use std::error::Error;

fn part1(lines: &[&str]) -> Result<usize, Box<dyn Error>> {
    let mut result = Vec::new();

    for line in lines {
        let values: Vec<_> = line.chars().filter(|x| x.is_numeric()).fuse().collect();
        let first = values.first().expect("no first element!!!");
        let second = values.last().expect("no last element!!!");

        result.push(format!("{}{}", first, second).parse()?)
    }

    Ok(result.iter().sum())
}

fn part2(lines: &[&str]) -> Result<usize, Box<dyn Error>> {
    // let mut result = Vec::new();
    for line in lines {
        let new_line = convert2numeric(line);
        println!("{new_line}")
    }

    Ok(0)
}

fn convert2numeric(input: &str) -> String {
    let mut result = String::new();
    for window_length in 1..=5 {
        for window in input.as_bytes().windows(window_length) {
            match (window_length, window) {
                (3, b"one") => result += "1",
                (3, b"two") => result += "2",
                (5, b"three") => result += "3",
                (4, b"four") => result += "4",
                (4, b"five") => result += "5",
                (3, b"six") => result += "6",
                (5, b"seven") => result += "7",
                (5, b"eight") => result += "8",
                (4, b"nine") => result += "8",
                (_, _) => continue,
            }
        }
    }
    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input_small2.txt");

    let input: Vec<_> = input.split_whitespace().collect();
    // let result1 = part1(&input)?;

    // println!("{result1:?}");

    let result2 = part2(&input);

    Ok(())
}
