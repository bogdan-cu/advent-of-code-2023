use day04::{read_input, Card};

pub fn main() -> color_eyre::Result<()> {
    let contents = read_input("./src/bin/input.txt")?;
    let card_values = contents
        .lines()
        .flat_map(Card::new)
        .flat_map(|card| card.value());

    let total: usize = card_values.sum();
    println!("pile of cards is worth a total of {} points", total);
    Ok(())
}
