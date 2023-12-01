use std::{
    fs::File,
    io::{Error, Read},
};

pub fn main() -> Result<(), Error> {
    let mut file = File::open("./src/bin/input.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let lines: Vec<&str> = contents.split('\n').collect();
    let total: u32 = lines
        .into_iter()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|chr| chr.to_digit(10)).collect();
            digits.first().unwrap_or(&0) * 10 + digits.last().unwrap_or(&0)
        })
        .sum();

    println!("Total is: {}", total);
    Ok(())
}
