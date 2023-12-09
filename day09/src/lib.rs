use std::io::Read;

use color_eyre::{eyre::eyre, Result};

pub fn read_input(path: &str) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn calculate_next(input: &[isize]) -> Result<isize> {
    if input.iter().all(|elem| *elem == 0) {
        return Ok(0);
    }
    let input_length = input.len();
    if input_length < 2 {
        return Err(eyre!("went to the bottom without finding a rule"));
    }
    let l1 = input.get(0..input_length - 1).unwrap();
    let l2 = input.get(1..input_length).unwrap();
    let r: Vec<isize> = l2
        .iter()
        .zip(l1.iter())
        .map(|(elem2, elem1)| *elem2 - *elem1)
        .collect();
    match calculate_next(&r) {
        Ok(val) => Ok(input.last().unwrap() + val),
        Err(e) => Err(e),
    }
}
pub fn calculate_prev(input: &[isize]) -> Result<isize> {
    if input.iter().all(|elem| *elem == 0) {
        return Ok(0);
    }
    let input_length = input.len();
    if input_length < 2 {
        return Err(eyre!("went to the bottom without finding a rule"));
    }
    let l1 = input.get(0..input_length - 1).unwrap();
    let l2 = input.get(1..input_length).unwrap();
    let r: Vec<isize> = l2
        .iter()
        .zip(l1.iter())
        .map(|(elem2, elem1)| *elem2 - *elem1)
        .collect();
    match calculate_prev(&r) {
        Ok(val) => Ok(input.first().unwrap() - val),
        Err(e) => Err(e),
    }
}
