use std::io::Read;

use color_eyre::{eyre::eyre, Result};

pub fn read_input(path: &str) -> Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_input_line(s: &str) -> Result<(&str, Vec<usize>)> {
    let mut parts = s.split_whitespace();
    let springs = parts.next().ok_or(eyre!("misshapen input"))?;
    let damaged_spring_list = parts.next().ok_or(eyre!("misshapen input"));
    let damaged_spring_list: Result<Vec<usize>> = damaged_spring_list?
        .split(',')
        .map(|num_str| {
            num_str
                .parse::<usize>()
                .map_err(|_| eyre!("found something that is not a number in the second section"))
        })
        .collect();
    if parts.next().is_some() {
        return Err(eyre!("line has more sections than expected"));
    }
    Ok((springs, damaged_spring_list?))
}

pub fn generate_constraints(s: &str) -> (usize, Vec<(usize, char)>) {
    let length = s.len();
    let constraints: Vec<(usize, char)> = s
        .chars()
        .enumerate()
        .filter(|(_, chr)| *chr == '.' || *chr == '#')
        .collect();
    (length, constraints)
}

pub fn generate_constraints_pt2(s: &str) -> (usize, Vec<(usize, char)>) {
    let length = s.len();
    let initial_constraints: Vec<(usize, char)> = s
        .chars()
        .enumerate()
        .filter(|(_, chr)| *chr == '.' || *chr == '#')
        .collect();

    let mut constraints = initial_constraints.clone();
    for i in 1..5 {
        constraints.append(
            &mut initial_constraints
                .iter()
                .map(|(idx, chr)| (*idx + length + i, *chr))
                .collect::<Vec<(usize, char)>>(),
        );
    }
    (length * 5 + 4, constraints)
}

pub fn generate_working_spring_sections(
    spring_count: usize,
    known_section_count: usize,
) -> Option<Vec<Vec<String>>> {
    let mut result = match generate(spring_count, known_section_count - 1) {
        Some(val) => val,
        None => Vec::<Vec<usize>>::new(),
    };
    result.append(&mut match generate(spring_count, known_section_count) {
        Some(val) => val,
        None => Vec::<Vec<usize>>::new(),
    });
    result.append(&mut match generate(spring_count, known_section_count + 1) {
        Some(val) => val,
        None => Vec::<Vec<usize>>::new(),
    });
    if result.is_empty() {
        return None;
    }
    let strings: Vec<Vec<String>> = result
        .iter()
        .map(|sections| {
            sections
                .iter()
                .map(|count| (0..*count).map(|_| '.').collect::<String>())
                .collect::<Vec<String>>()
        })
        .collect();

    Some(strings)
}

fn generate(count: usize, length: usize) -> Option<Vec<Vec<usize>>> {
    if count == 0 || length == 0 || count < length {
        return None;
    }
    if length == 1 {
        return Some(vec![vec![count]]);
    }
    Some(
        (1..=count - length + 1)
            .flat_map(|val| {
                generate(count - val, length - 1).map(|generated_arrays| {
                    generated_arrays
                        .into_iter()
                        .map(|mut array| {
                            array.push(val);
                            array
                        })
                        .collect::<Vec<Vec<usize>>>()
                })
            })
            .fold(Vec::<Vec<usize>>::new(), |mut acc, mut elem| {
                acc.append(&mut elem);
                acc
            }),
    )
}

pub fn string_is_valid(s: &str, constraints: &[(usize, char)]) -> bool {
    !s.chars().enumerate().any(|(idx, chr)| {
        constraints
            .iter()
            .any(|(idx_c, chr_c)| idx == *idx_c && chr != *chr_c)
    })
}
