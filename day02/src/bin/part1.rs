use day02::{Extraction, Game};
use std::io::Read;

pub fn main() -> color_eyre::Result<()> {
    let mut file = std::fs::File::open("./src/bin/input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let load = Extraction::new(12, 13, 14);
    let total_of_games: usize = contents
        .lines()
        .flat_map(Game::parse_from)
        .filter(|game| Game::validate(game, load))
        .map(|game| day02::Game::id(&game))
        .sum();

    println!("total sum of games is {}", total_of_games);

    Ok(())
}
