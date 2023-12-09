use color_eyre::eyre::{eyre, Ok, Result};
use day09::{calculate_prev, read_input};

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;
    let lines = contents.lines();

    let sequences: Result<Vec<Vec<isize>>> = lines
        .map(|s| {
            let r: Result<Vec<isize>> = s
                .split_whitespace()
                .map(|num_str| {
                    num_str
                        .parse::<isize>()
                        .map_err(|_| eyre!("could not parse input {}", num_str))
                })
                .collect();
            r
        })
        .collect();
    let sequences = sequences?;

    let result: Result<isize> = sequences
        .iter()
        .map(|sequence| calculate_prev(sequence))
        .sum();
    println!("total is {}", result?);

    Ok(())
}
