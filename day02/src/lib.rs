use std::cmp::max;
use std::fmt::Error;

#[derive(Debug, Clone, Copy)]
pub struct Extraction {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug, Clone)]
pub struct Game {
    id: usize,
    extractions: Vec<Extraction>,
}

impl Extraction {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Extraction { red, green, blue }
    }

    fn parse_from(input: &str) -> color_eyre::Result<Self> {
        let result = input
            .split(", ")
            .map(|s| {
                let red = Self::parse_color(s, "red").unwrap_or(0);
                let green = Self::parse_color(s, "green").unwrap_or(0);
                let blue = Self::parse_color(s, "blue").unwrap_or(0);
                (red, green, blue)
            })
            .fold((0, 0, 0), |acc, elem| {
                (acc.0 + elem.0, acc.1 + elem.1, acc.2 + elem.2)
            });
        Ok(Extraction {
            red: result.0,
            green: result.1,
            blue: result.2,
        })
    }

    fn parse_color(input: &str, color: &str) -> color_eyre::Result<usize> {
        let val = input.strip_suffix(color).ok_or(Error)?;
        val.trim().parse::<usize>().map_err(|err| err.into())
    }

    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl Game {
    pub fn parse_from(input: &str) -> color_eyre::Result<Self> {
        let mut result = input.split(": ");

        let code = result
            .next()
            .and_then(|header| header.strip_prefix("Game "))
            .ok_or(Error)?;
        let id = code.parse::<usize>()?;

        let extraction_data = result.next().ok_or(Error)?;
        let extractions: Vec<Extraction> = extraction_data
            .split("; ")
            .flat_map(Extraction::parse_from)
            .collect();

        Ok(Game { id, extractions })
    }

    pub fn validate(&self, load: Extraction) -> bool {
        let extractions = self.extractions.clone();
        !extractions.into_iter().any(|extraction| {
            extraction.red > load.red
                || extraction.green > load.green
                || extraction.blue > load.blue
        })
    }

    pub fn minimum_load(&self) -> Extraction {
        let extractions = self.extractions.clone();
        extractions
            .into_iter()
            .fold(Extraction::new(0, 0, 0), |acc, elem| {
                Extraction::new(
                    max(acc.red, elem.red),
                    max(acc.green, elem.green),
                    max(acc.blue, elem.blue),
                )
            })
    }

    pub fn id(&self) -> usize {
        self.id
    }
}
