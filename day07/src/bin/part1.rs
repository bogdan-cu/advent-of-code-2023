use color_eyre::{eyre::eyre, Result};
use day07::read_input;
use day07::Hand;

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;
    let lines = contents.lines();

    let parsed_input: Result<Vec<(&str, usize)>> = lines
        .map(|line| {
            let mut strs = line.split_whitespace();
            let cards = strs.next().ok_or(eyre!("malformed input"))?;
            let bid = strs
                .next()
                .ok_or(eyre!("malformed input"))
                .map(|bid_str| bid_str.parse::<usize>())
                .map_err(|_| eyre!("could not parse bid"))??;
            Ok((cards, bid))
        })
        .collect();

    let mut cards: Vec<Hand> = parsed_input?
        .iter()
        .flat_map(|(cards, bid)| Hand::new(cards, *bid))
        .collect();
    cards.sort();

    let result: usize = cards
        .iter()
        .enumerate()
        .map(|(idx, hand)| hand.get_bid() * (idx + 1))
        .sum();
    println!("total is {}", result);

    Ok(())
}
