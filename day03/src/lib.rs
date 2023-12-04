use std::{cmp::min, io::Read};

use color_eyre::eyre::eyre;

pub fn read_input(path: &str) -> color_eyre::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    if !contents.is_ascii() {
        Err(eyre!("this text contains non-ascii values"))
    } else {
        Ok(contents)
    }
}

//part one

fn contains_symbol(line: &[u8], start_idx: usize, end_idx: usize) -> bool {
    let mut lower_bound = 0;
    if start_idx > 0 {
        lower_bound = start_idx - 1;
    }
    let upper_bound = min(line.len(), end_idx + 2);

    line.get(lower_bound..upper_bound)
        .unwrap()
        .iter()
        .any(|chr| !(chr.is_ascii_alphanumeric() || chr.is_ascii_whitespace() || *chr == b'.'))
}

fn get_numbers(s: &str) -> Vec<(usize, usize, usize)> {
    let chars = s.as_bytes();
    let mut numbers: Vec<(usize, usize, usize)> = Vec::new();

    for (idx, char) in chars.iter().enumerate() {
        if char.is_ascii_digit() {
            if idx == 0 || !chars[idx - 1].is_ascii_digit() {
                numbers.push(((*char - 48) as usize, idx, idx));
            } else {
                let elem = numbers.last_mut().unwrap();
                elem.0 = elem.0 * 10 + (*char - 48) as usize;
                elem.2 = idx;
            }
        }
    }

    numbers
}

pub fn parse_alone(line: &str) -> usize {
    let chrs = line.as_bytes();

    let mut sum = 0;
    for (number, start_idx, end_idx) in get_numbers(line) {
        if contains_symbol(chrs, start_idx, end_idx) {
            sum += number;
        }
    }

    sum
}

pub fn parse_with_one_adjacent_line(line: &str, adjacent_line: &str) -> usize {
    let line_chrs = line.as_bytes();
    let adj_line_chrs = adjacent_line.as_bytes();

    let mut sum = 0;
    for (number, start_idx, end_idx) in get_numbers(line) {
        if contains_symbol(line_chrs, start_idx, end_idx)
            || contains_symbol(adj_line_chrs, start_idx, end_idx)
        {
            sum += number;
        }
    }

    sum
}

pub fn parse_with_two_adjacent_lines(line: &str, prev_line: &str, next_line: &str) -> usize {
    let line_chrs = line.as_bytes();
    let prev_line_chrs = prev_line.as_bytes();
    let next_line_chrs = next_line.as_bytes();

    let mut sum = 0;
    for (number, start_idx, end_idx) in get_numbers(line) {
        if contains_symbol(line_chrs, start_idx, end_idx)
            || contains_symbol(prev_line_chrs, start_idx, end_idx)
            || contains_symbol(next_line_chrs, start_idx, end_idx)
        {
            sum += number;
        }
    }

    sum
}

//part two

pub fn contains_wheel(line: &str) -> Option<Vec<usize>> {
    let mut positions = Vec::new();
    for (idx, char) in line.as_bytes().iter().enumerate() {
        if *char == b'*' {
            positions.push(idx);
        }
    }

    if positions.is_empty() {
        None
    } else {
        Some(positions)
    }
}

pub fn parse_line(line: &str, pos: usize) -> Vec<usize> {
    let numbers = get_numbers(line);
    numbers
        .into_iter()
        .filter(|(_, start_idx, end_idx)| {
            (pos > 0 && *end_idx == pos - 1)
                || (pos < *start_idx && *start_idx == pos + 1)
                || (*start_idx <= pos && pos <= *end_idx)
        })
        .map(|(number, _, _)| number)
        .collect()
}
