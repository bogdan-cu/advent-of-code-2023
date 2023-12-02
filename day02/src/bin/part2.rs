use day02::Game;
use std::io::Read;

pub fn main() -> color_eyre::Result<()> {
    let mut file = std::fs::File::open("./src/bin/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let sum_of_powers: usize = contents
        .lines()
        .flat_map(Game::parse_from)
        .map(|game| game.minimum_load())
        .map(|load| load.power())
        .sum();

    println!("total sum of powers is {}", sum_of_powers);

    Ok(())
}
