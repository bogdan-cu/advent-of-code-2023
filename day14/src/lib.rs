use std::io::Read;

use color_eyre::{eyre::eyre, Result};
use ndarray::Array2;

pub fn read_input(path: &str) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn to_array(s: &str) -> Result<Array2<char>> {
    let lines = s.lines().map(|s| s.trim()).collect::<Vec<&str>>();
    let x = lines.len();
    let y = lines
        .iter()
        .map(|line| line.len())
        .max()
        .ok_or(eyre!("found empty line"))?;
    let mut arr = Array2::from_elem((x, y), '.');

    for (idx_x, row) in lines.iter().enumerate() {
        for (idx_y, chr) in row.chars().enumerate() {
            if chr == '#' || chr == 'O' || chr == '.' {
                *arr.get_mut((idx_x, idx_y)).unwrap() = chr;
            } else {
                return Err(eyre!("found odd character in input"));
            }
        }
    }

    Ok(arr)
}
