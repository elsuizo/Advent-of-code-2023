use std::collections::HashMap;
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

// TODO(elsuizo: 2023-12-04): mejorar esto que quedo re feo...
fn part2(lines: &[&str]) -> Result<usize, Box<dyn Error>> {
    let mut digits = HashMap::new();
    digits.insert("one", "1");
    digits.insert("two", "2");
    digits.insert("three", "3");
    digits.insert("four", "4");
    digits.insert("five", "5");
    digits.insert("six", "6");
    digits.insert("seven", "7");
    digits.insert("eight", "8");
    digits.insert("nine", "9");
    let mut result = Vec::new();
    for line in lines {
        let mut aux = Vec::new();
        for (index, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                aux.push(c.to_string());
            } else {
                for (index_digits, &digit) in digits.keys().enumerate() {
                    if line[index..].starts_with(digit) {
                        aux.push(digits.get(&digit).unwrap().to_string())
                    }
                }
            }
        }
        // TODO(elsuizo: 2023-12-04): esto se podria simplificar mucho
        let a = aux.first().unwrap().to_owned() + aux.last().unwrap();
        result.push(a);
    }
    // TODO(elsuizo: 2023-12-04): esto se ve re feo
    let r = result
        .iter()
        .map(|x| x.as_str().parse::<usize>().unwrap())
        .sum();
    Ok(r)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");

    let input: Vec<_> = input.split_whitespace().collect();
    // let result1 = part1(&input)?;

    // println!("{result1:?}");
    let result2 = part2(&input)?;
    println!("{result2}");

    Ok(())
}
