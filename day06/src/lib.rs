use std::io::Read;

pub fn read_input(path: &str) -> color_eyre::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn count_winning_strategies(timer: usize, distance: usize) -> usize {
    let result: Vec<usize> = (0..=timer)
        .filter(|time| *time * (timer - *time) > distance)
        .collect();
    result.len()
}

pub fn count_winning_strategies_efficient(timer: usize, distance: usize) -> usize {
    //as seen in the brute force solution above, solutions to the inequality are located in the interval determined
    //by the two solutions of the quadratic equation
    let t = timer as f64;
    let d = distance as f64;
    let sol1 = (t - f64::sqrt(f64::powi(t, 2) - 4.00 * d)) / 2.00;
    let sol2 = (t + f64::sqrt(f64::powi(t, 2) - 4.00 * d)) / 2.00;

    sol2.floor().min(t) as usize - sol1.ceil().max(0.00) as usize + 1
}
