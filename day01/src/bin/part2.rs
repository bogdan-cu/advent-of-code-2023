use std::{
    fs::File,
    io::{Error, Read},
};

pub fn main() -> Result<(), Error> {
    let mut file = File::open("./src/bin/input.txt")?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let spelled_out_digits = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let lines: Vec<&str> = contents.split('\n').collect();

    let total: usize = lines
        .into_iter()
        .map(|line| {
            let mut first_digit = 0;
            let mut last_digit = 0;

            let mut idx = 0;
            while first_digit == 0 && idx < line.len() {
                if let Some(substr) = line.get(idx..) {
                    if substr.starts_with(|chr: char| chr.is_numeric()) {
                        let first_chr = substr.chars().next().unwrap();
                        first_digit = first_chr.to_digit(10).unwrap() as usize;
                        break;
                    }
                    for (value, text) in spelled_out_digits.iter().enumerate() {
                        if substr.starts_with(text) {
                            first_digit = value;
                            break;
                        };
                    }
                    idx += 1;
                } else {
                    idx += 1;
                    continue;
                }
            }

            idx = line.len() - 1;
            while last_digit == 0 {
                if let Some(substr) = line.get(idx..) {
                    if substr.starts_with(|chr: char| chr.is_numeric()) {
                        let first_chr = substr.chars().next().unwrap();
                        last_digit = first_chr.to_digit(10).unwrap() as usize;
                        break;
                    }
                    for (value, text) in spelled_out_digits.iter().enumerate() {
                        if substr.starts_with(text) {
                            last_digit = value;
                            break;
                        };
                    }
                    idx -= 1;
                } else {
                    idx -= 1;
                    continue;
                }
            }

            first_digit * 10 + last_digit
        })
        .sum();

    println!("Total is: {}", total);
    Ok(())
}
