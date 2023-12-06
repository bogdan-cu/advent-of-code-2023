use color_eyre::eyre::eyre;
use day06::{count_winning_strategies, read_input};

pub fn main() -> color_eyre::Result<()> {
    let contents = read_input("./src/input.txt")?;
    let mut lines = contents.lines();
    let timers: Vec<usize> = lines
        .next()
        .ok_or(eyre!("malformed input"))?
        .strip_prefix("Time:")
        .ok_or(eyre!("not a record of race times"))?
        .split_whitespace()
        .flat_map(|num_str| num_str.parse::<usize>())
        .collect();
    let distances: Vec<usize> = lines
        .next()
        .ok_or(eyre!("malformed input"))?
        .strip_prefix("Distance:")
        .ok_or(eyre!("not a record of race record distances"))?
        .split_whitespace()
        .flat_map(|num_str| num_str.parse::<usize>())
        .collect();

    let races = timers.iter().zip(distances.iter());
    let total: usize = races
        .map(|(timer, distance)| count_winning_strategies(*timer, *distance))
        .product();
    println!("result is {}", total);

    Ok(())
}
