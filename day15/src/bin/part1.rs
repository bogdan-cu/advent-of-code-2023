use color_eyre::eyre::Result;
use day15::{h_a_s_h, read_input};

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;

    let parsed_contents = contents
        .split(',')
        .map(h_a_s_h)
        .collect::<Result<Vec<usize>>>();

    println!("{}", parsed_contents?.iter().sum::<usize>());
    Ok(())
}
