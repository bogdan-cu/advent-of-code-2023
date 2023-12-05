use color_eyre::eyre::eyre;
use day05::{read_input, Transformation};

pub fn main() -> color_eyre::Result<()> {
    let contents = read_input("./src/input.txt")?;
    let mut sections = contents.split("\n\n");

    let seed_section = sections.next().ok_or(eyre!("malformed input"))?;
    let seeds: Vec<usize> = seed_section
        .strip_prefix("seeds: ")
        .map(|s| s.split_whitespace())
        .ok_or(eyre!("malformed input"))?
        .flat_map(|s| s.parse::<usize>())
        .collect();
    //this is the array of objects that contains the iterative maps from seed to location
    let mut obj = seeds.clone();

    while let Some(conversion_section) = sections.next() {
        let maps = Transformation::parse(conversion_section)?;
        let new_obj: Vec<usize> = obj
            .into_iter()
            .map(|obj| Transformation::transform(obj, &maps))
            .collect();
        obj = new_obj;
    }

    let closest_location = obj
        .into_iter()
        .min()
        .ok_or(eyre!("operation to determine closest location failed"))?;
    println!("the closest location is {}", closest_location);

    Ok(())
}
