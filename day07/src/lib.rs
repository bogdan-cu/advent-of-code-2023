use color_eyre::{eyre::eyre, Result};
use std::{cmp::Ordering, collections::HashMap, io::Read};

pub fn read_input(path: &str) -> color_eyre::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Debug)]
pub struct Hand {
    contents: Vec<usize>,
    hand_type: usize,
    bid: usize,
}

impl Hand {
    fn parse_card(card: char) -> Result<usize> {
        match card {
            '2' => Ok(2),
            '3' => Ok(3),
            '4' => Ok(4),
            '5' => Ok(5),
            '6' => Ok(6),
            '7' => Ok(7),
            '8' => Ok(8),
            '9' => Ok(9),
            'T' => Ok(10),
            'J' => Ok(11),
            'Q' => Ok(12),
            'K' => Ok(13),
            'A' => Ok(14),
            x => Err(eyre!("Invalid card input found: {}", x)),
        }
    }

    fn calculate_value(frequencies: &Vec<usize>) -> Result<usize> {
        match frequencies.len() {
            5 => Ok(1),
            4 => Ok(2),
            3 => {
                if frequencies.iter().any(|elem| *elem == 3) {
                    return Ok(4);
                }
                let twos: Vec<usize> = frequencies
                    .iter()
                    .filter(|elem| **elem == 2)
                    .copied()
                    .collect();
                if twos.len() == 2 {
                    return Ok(3);
                }
                Err(eyre!("malformed hand of cards"))
            }
            2 => {
                if frequencies.iter().any(|elem| *elem == 4) {
                    return Ok(6);
                }
                if frequencies.iter().any(|elem| *elem == 3)
                    && frequencies.iter().any(|elem| *elem == 2)
                {
                    return Ok(5);
                }
                Err(eyre!("malformed hand of cards"))
            }
            1 => Ok(7),
            _ => Err(eyre!("malformed hand of cards")),
        }
    }

    pub fn new(cards: &str, bid: usize) -> Result<Self> {
        let cards: Result<Vec<usize>> = cards.chars().map(Self::parse_card).collect();
        let cards = cards?;

        let mut card_frequencies: HashMap<usize, usize> = HashMap::new();
        for card in cards.iter() {
            *card_frequencies.entry(*card).or_default() += 1;
        }
        let frequencies: Vec<usize> = card_frequencies.values().copied().collect();
        let value = Self::calculate_value(&frequencies)?;

        Ok(Self {
            contents: cards,
            hand_type: value,
            bid,
        })
    }

    pub fn get_bid(&self) -> usize {
        self.bid
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }
        let pairs: Vec<(&usize, &usize)> = self
            .contents
            .iter()
            .zip(other.contents.iter())
            .filter(|(fst, lst)| *fst != *lst)
            .collect();
        pairs.first().is_none()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Hand {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type < other.hand_type {
            return Ordering::Less;
        }
        if self.hand_type > other.hand_type {
            return Ordering::Greater;
        }
        //pairs of different cards across hands
        let pairs: Vec<(&usize, &usize)> = self
            .contents
            .iter()
            .zip(other.contents.iter())
            .filter(|(fst, lst)| *fst != *lst)
            .collect();
        if let Some((fst, lst)) = pairs.first() {
            if *fst < *lst {
                return Ordering::Less;
            }
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}
