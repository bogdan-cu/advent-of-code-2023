use std::io::Read;

use color_eyre::{eyre::eyre, Result};
use ndarray::Array2;

pub fn read_input(path: &str) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn to_array(s: &str) -> Result<Array2<u8>> {
    let lines = s.lines().map(|s| s.trim()).collect::<Vec<&str>>();
    let x = lines.len();
    let y = lines
        .iter()
        .map(|line| line.len())
        .max()
        .ok_or(eyre!("found empty line"))?;
    let mut x = Array2::from_elem((x, y), 0_u8);

    for (idx_x, row) in lines.iter().enumerate() {
        for (idx_y, chr) in row.chars().enumerate() {
            if chr == '#' {
                *x.get_mut((idx_x, idx_y)).unwrap() = 1;
            }
        }
    }

    Ok(x)
}

pub fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold(0, |acc, &b| (acc << 1) + b as u32)
}

pub fn find_reflection(v: &[u32]) -> Option<usize> {
    for i in 0..v.len() / 2 {
        let s = v.get(..=i).unwrap();
        let r = v.get(i + 1..).unwrap();
        if s.iter().rev().zip(r.iter()).all(|(e1, e2)| *e1 == *e2) {
            return Some(s.len());
        }
    }
    for i in v.len() / 2 + 1..v.len() {
        let s = v.get(i..).unwrap();
        let r = v.get(..i).unwrap();
        if s.iter().zip(r.iter().rev()).all(|(e1, e2)| *e1 == *e2) {
            return Some(r.len());
        }
    }
    None
}

pub fn find_reflection_alt(v: &[u32]) -> Option<Vec<usize>> {
    let mut reflections = Vec::<usize>::new();
    for i in 0..v.len() / 2 {
        let s = v.get(..=i).unwrap();
        let r = v.get(i + 1..).unwrap();
        if s.iter().rev().zip(r.iter()).all(|(e1, e2)| *e1 == *e2) {
            reflections.push(s.len());
        }
    }
    for i in v.len() / 2 + 1..v.len() {
        let s = v.get(i..).unwrap();
        let r = v.get(..i).unwrap();
        if s.iter().zip(r.iter().rev()).all(|(e1, e2)| *e1 == *e2) {
            reflections.push(r.len());
        }
    }
    if reflections.is_empty() {
        return None;
    }
    Some(reflections)
}
