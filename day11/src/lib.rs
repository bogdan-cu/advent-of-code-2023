use std::io::Read;

use color_eyre::{eyre::Ok, Result};

pub fn read_input(path: &str) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn get_distance(fst: &(usize, usize), lst: &(usize, usize)) -> isize {
    (fst.0 as isize - lst.0 as isize).abs() + (fst.1 as isize - lst.1 as isize).abs()
}

pub fn get_distance_pt2(
    fst: &(usize, usize),
    lst: &(usize, usize),
    line_expansions: &[usize],
    column_expansions: &[usize],
    expansion_size: usize,
) -> usize {
    let x_min = fst.0.min(lst.0);
    let x_max = fst.0.max(lst.0);
    let y_min = fst.1.min(lst.1);
    let y_max = fst.1.max(lst.1);

    let x_expansions = line_expansions
        .iter()
        .filter(|expansion| **expansion > x_min && **expansion < x_max)
        .count();
    let y_expansions = column_expansions
        .iter()
        .filter(|expansion| **expansion > y_min && **expansion < y_max)
        .count();

    x_expansions * (expansion_size - 1) + x_max - x_min
        + y_expansions * (expansion_size - 1)
        + y_max
        - y_min
}
