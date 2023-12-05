use std::ops::Range;

use color_eyre::eyre::eyre;
use day05::{read_input, Transformation};

pub fn main() -> color_eyre::Result<()> {
    let contents = read_input("./src/input.txt")?;
    let mut sections = contents.split("\n\n");

    let seed_section = sections.next().ok_or(eyre!("malformed input"))?;
    let seed_section_values: Vec<usize> = seed_section
        .strip_prefix("seeds: ")
        .map(|s| s.split_whitespace())
        .ok_or(eyre!("malformed seed input"))?
        .flat_map(|s| s.parse::<usize>())
        .collect();
    if !seed_section_values.len() % 2 == 0 {
        return Err(eyre!("malformed seed input"));
    }
    let mut range_start: Vec<usize> = Vec::new();
    let mut range_length: Vec<usize> = Vec::new();
    for (idx, val) in seed_section_values.iter().enumerate() {
        if idx % 2 == 0 {
            range_start.push(*val);
        } else {
            range_length.push(*val);
        }
    }
    let seed_ranges: Vec<Range<usize>> = range_start
        .iter()
        .zip(range_length.iter())
        .map(|(s, l)| Range {
            start: *s,
            end: *s + *l,
        })
        .collect();

    //this is the array of objects that contains the iterative maps from seed to location
    let mut obj = seed_ranges;

    while let Some(conversion_section) = sections.next() {
        let maps = Transformation::parse(conversion_section)?;
        let new_obj: Vec<Range<usize>> = obj
            .into_iter()
            .flat_map(|obj| Transformation::transform_range(obj, &maps))
            .collect();
        obj = new_obj;
    }

    let closest_location = obj
        .into_iter()
        .map(|val_range| val_range.start)
        .min()
        .ok_or(eyre!("operation to determine closest location failed"))?;
    println!("the closest location is {}", closest_location);
    Ok(())
}
