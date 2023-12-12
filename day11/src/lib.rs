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
