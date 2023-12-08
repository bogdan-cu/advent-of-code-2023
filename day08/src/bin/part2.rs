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

    let curr_location: Vec<&Map> = maps
        .iter()
        .filter(|elem| elem.get_location().ends_with('A'))
        .collect();

    let steps: Result<Vec<usize>> = curr_location
        .iter()
        .map(|location| {
            let mut location = location.to_owned();
            let mut steps = 0;
            for instruction in instructions.iter().cycle() {
                if location.get_location().ends_with('Z') {
                    break;
                }
                if let Some(new_location) = maps
                    .iter()
                    .find(|elem| elem.get_location() == location.get_next(instruction))
                {
                    location = new_location;
                    steps += 1;
                } else {
                    return Err(eyre!("could not find next step"));
                }
            }
            Ok(steps)
        })
        .collect();

    let steps = steps?
        .iter()
        .fold(1, |acc, elem| num::Integer::lcm(&acc, elem));

    println!("it takes {} steps to reach destination", steps);

    Ok(())
}
