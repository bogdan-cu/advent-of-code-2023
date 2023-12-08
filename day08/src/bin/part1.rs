use color_eyre::eyre::{eyre, Result};
use day08::{parse_instructions, read_input, Map};

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;
    let mut results = contents.split("\n\n");

    let instructions = parse_instructions(
        results
            .next()
            .ok_or(eyre!("could not find instructions section"))?,
    )?;

    let maps: Result<Vec<Map>> = results
        .next()
        .ok_or(eyre!("could not find maps section"))?
        .lines()
        .map(Map::new)
        .collect();
    let maps = maps?;

    let mut curr_location = maps
        .iter()
        .find(|elem| elem.get_location() == "AAA")
        .ok_or(eyre!("could not find starting point"))?;
    let mut steps = 0;

    for instruction in instructions.iter().cycle() {
        if curr_location.get_location() == "ZZZ" {
            break;
        }
        curr_location = maps
            .iter()
            .find(|elem| elem.get_location() == curr_location.get_next(instruction))
            .ok_or(eyre!("could not find a map of the next step"))?;
        steps += 1;
    }

    println!("it takes {} steps to reach destination", steps);

    Ok(())
}
