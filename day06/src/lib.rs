use std::io::Read;

pub fn read_input(path: &str) -> color_eyre::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn count_winning_strategies(timer: usize, distance: usize) -> usize {
    let result: Vec<usize> = (0..=timer)
        .filter(|time| (*time as isize * (timer as isize - *time as isize) - distance as isize) > 0)
        .collect();
    result.len()
}
