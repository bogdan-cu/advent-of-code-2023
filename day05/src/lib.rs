use std::io::Read;

use color_eyre::eyre::eyre;

pub fn read_input(path: &str) -> color_eyre::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[derive(Clone, Copy)]
pub struct Transformation {
    src_start: usize,
    dest_start: usize,
    range: usize,
}

impl Transformation {
    fn new(s: &str) -> color_eyre::Result<Self> {
        let numbers: Vec<usize> = s
            .split_whitespace()
            .flat_map(|num_str| num_str.parse::<usize>())
            .collect();

        if numbers.len() != 3 {
            return Err(eyre!("could not parse line into map"));
        }

        let map = Transformation {
            src_start: numbers[1],
            dest_start: numbers[0],
            range: numbers[2],
        };
        Ok(map)
    }

    pub fn parse(s: &str) -> color_eyre::Result<Vec<Self>> {
        let maps: Vec<Transformation> = s
            .lines()
            .filter(|line| !line.ends_with("map:"))
            .flat_map(Transformation::new)
            .collect();

        if maps.is_empty() {
            return Err(eyre!("could not parse section into maps"));
        }
        Ok(maps)
    }

    fn range_contains(&self, obj: usize) -> bool {
        obj >= self.src_start && obj < self.src_start + self.range
    }

    fn convert(&self, obj: usize) -> usize {
        self.dest_start + (obj - self.src_start)
    }

    pub fn transform(input: usize, maps: &[Self]) -> usize {
        let map = maps.iter().find(|map| map.range_contains(input));

        match map {
            Some(map) => map.convert(input),
            None => input,
        }
    }
}
