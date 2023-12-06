use color_eyre::eyre::eyre;
use day06::{count_winning_strategies_efficient, read_input};

pub fn main() -> color_eyre::Result<()> {
    let contents = read_input("./src/input.txt")?;
    let mut lines = contents.lines();
    let timer = lines
        .next()
        .ok_or(eyre!("malformed input"))?
        .strip_prefix("Time:")
        .ok_or(eyre!("not a record of race time"))?
        .replace(' ', "")
        .parse::<usize>()
        .map_err(|_| eyre!("could not parse timer"))?;
    let distance = lines
        .next()
        .ok_or(eyre!("malformed input"))?
        .strip_prefix("Distance:")
        .ok_or(eyre!("not a record of race record distance"))?
        .replace(' ', "")
        .parse::<usize>()
        .map_err(|_| eyre!("could not parse distance"))?;

    let result_efficient = count_winning_strategies_efficient(timer, distance);
    println!("result is {}", result_efficient);
    Ok(())
}
