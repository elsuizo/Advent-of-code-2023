use std::error::Error;

enum Cubes {
    Red,
    Green,
    Blue,
}

struct Game {
    id: usize,
    elements: Vec<Cubes>,
}

// impl Game {
//     fn new_from(input: &str) -> Self {
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input_small.txt")?;

    let input: Vec<_> = input.split_terminator("\n").collect();

    let g1: Vec<_> = input[0].split(&[';', ':']).collect();

    println!("{g1:?}");

    Ok(())
}
