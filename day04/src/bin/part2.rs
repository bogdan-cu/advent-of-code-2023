use day04::{read_input, Card};

pub fn main() -> color_eyre::Result<()> {
    let contents = read_input("./src/bin/input.txt")?;
    let mut cards: Vec<(Card, usize)> = contents
        .lines()
        .flat_map(Card::new)
        .map(|card| (card, 1_usize))
        .collect();

    let len = cards.len();
    let mut idx = 0;
    while idx < len {
        let entry = &cards[idx];
        let mut value = entry.0.true_value();
        let count = entry.1;

        let mut next = 1;
        while value > 0 && idx + next < len {
            let next_entry = &mut cards[idx + next];
            next_entry.1 += count;
            next += 1;
            value -= 1;
        }
        idx += 1;
    }

    let total: usize = cards.into_iter().map(|entry| entry.1).sum();

    println!("pile of cards contains a total of {} cards", total);
    Ok(())
}
