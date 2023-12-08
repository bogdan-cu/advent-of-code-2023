use std::io::Read;

use color_eyre::{
    eyre::{eyre, Ok},
    Result,
};

pub fn read_input(path: &str) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_instructions(input: &str) -> Result<Vec<bool>> {
    let result: Result<Vec<bool>> = input
        .chars()
        .map(|chr| match chr {
            'L' => Ok(false),
            'R' => Ok(true),
            _ => Err(eyre!(
                "found something else besides L and R in the instruction string"
            )),
        })
        .collect();
    result
}

#[derive(Debug)]
pub struct Map<'a> {
    src: &'a str,
    left_dest: &'a str,
    right_dest: &'a str,
}

impl<'a> Map<'a> {
    pub fn new(input: &'a str) -> Result<Self> {
        let mut parts = input.split('=');
        let mut map = Map {
            src: "",
            left_dest: "",
            right_dest: "",
        };
        if let Some(start) = parts.next() {
            map.src = start.trim();
        } else {
            return Err(eyre!("could not parse source"));
        }
        if let Some(dest) = parts.next() {
            let s = dest
                .trim()
                .strip_prefix('(')
                .and_then(|dest| dest.strip_suffix(')'))
                .ok_or(eyre!("could not find properly formatted destinations"));
            let destinations: Vec<&str> = s?.split(',').map(|dest_str| dest_str.trim()).collect();
            if destinations.len() != 2 {
                return Err(eyre!("found an improper number of destinations"));
            }
            map.left_dest = destinations.first().unwrap();
            map.right_dest = destinations.last().unwrap();
        } else {
            return Err(eyre!("could not parse destinations"));
        }

        Ok(map)
    }

    pub fn get_location(&self) -> &str {
        self.src
    }

    pub fn get_next(&self, instruction: &bool) -> &str {
        match *instruction {
            false => self.left_dest,
            true => self.right_dest,
        }
    }
}
