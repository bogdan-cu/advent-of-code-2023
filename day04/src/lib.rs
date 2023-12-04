use std::io::Read;

use color_eyre::eyre::eyre;

pub fn read_input(path: &str) -> color_eyre::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub struct Card {
    id: usize,
    numbers: Vec<usize>,
    winners: Vec<usize>,
}

impl Card {
    pub fn new(s: &str) -> color_eyre::Result<Self> {
        let err = eyre!("this is not a well-formed card string");
        let mut results = s.split(':');
        let mut card = Card {
            id: 0,
            numbers: Vec::new(),
            winners: Vec::new(),
        };

        if let Some(card_code) = results.next() {
            if let Some(id_str) = card_code.trim().strip_prefix("Card ") {
                card.id = id_str.trim().parse::<usize>().map_err(|_| err)?
            } else {
                return Err(err);
            }
        } else {
            return Err(err);
        }

        let mut number_section = results
            .next()
            .ok_or(eyre!("there are no numbers on this card line"))?
            .split('|');

        if let Some(s) = number_section.next() {
            card.numbers = Self::get_numbers(s);
        }

        if let Some(s) = number_section.next() {
            card.winners = Self::get_numbers(s);
        }

        Ok(card)
    }

    fn get_numbers(s: &str) -> Vec<usize> {
        s.split(' ')
            .map(|substr| substr.trim())
            .flat_map(|num_str| num_str.parse::<usize>())
            .collect()
    }

    pub fn value(&self) -> color_eyre::Result<usize> {
        let mut numbers = self.numbers.clone();
        let winners = self.winners.clone();
        numbers.retain(|num| winners.contains(num));

        let base: usize = 2;
        let exp: u32 = match numbers.len().try_into() {
            Ok(val) => val,
            Err(_) => return Err(eyre!("too many numbers on the card")),
        };
        if exp == 0 {
            Ok(0)
        } else {
            Ok(base.pow(exp - 1))
        }
    }
}
