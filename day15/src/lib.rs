use std::io::Read;

use color_eyre::{eyre::eyre, Result};

pub fn read_input(path: &str) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn h_a_s_h(s: &str) -> Result<usize> {
    let mut total = 0_usize;
    if s.chars().any(|chr| !chr.is_ascii()) {
        return Err(eyre!("non-ascii characters found in input str {}", s));
    }
    for elem in s.as_bytes().iter() {
        total += *elem as usize;
        total *= 17;
        total %= 256;
    }
    Ok(total)
}
