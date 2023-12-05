use std::{io::Read, ops::Range};

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

    fn convert_range(&self, obj: &Range<usize>) -> Range<usize> {
        Range {
            start: self.convert(obj.start),
            end: self.convert(obj.end),
        }
    }

    pub fn transform(input: usize, maps: &[Self]) -> usize {
        let map = maps.iter().find(|map| map.range_contains(input));

        match map {
            Some(map) => map.convert(input),
            None => input,
        }
    }

    fn intersect(&self, obj: &Range<usize>) -> Option<(Range<usize>, Vec<Range<usize>>)> {
        let map_start = self.src_start;
        let map_end = self.src_start + self.range;
        if map_start < obj.end && obj.start < map_end {
            let intersection = Range {
                start: obj.start.max(map_start),
                end: obj.end.min(map_end),
            };
            let left = Range {
                start: obj.start,
                end: intersection.start,
            };
            let right = Range {
                start: intersection.end,
                end: obj.end,
            };
            let mut unconverted_output: Vec<Range<usize>> = Vec::new();
            if !left.is_empty() {
                unconverted_output.push(left);
            }
            if !right.is_empty() {
                unconverted_output.push(right);
            }
            return Some((self.convert_range(&intersection), unconverted_output));
        }
        None
    }

    pub fn transform_range(input: Range<usize>, maps: &[Self]) -> Vec<Range<usize>> {
        let mut input_ranges = vec![input];
        //let mut input_ranges: VecDeque<Range<usize>> = VecDeque::new();
        let mut output: Vec<Range<usize>> = Vec::new();
        let mut remains: Vec<Range<usize>> = Vec::new();
        for map in maps {
            remains.clear();
            for input_range in &input_ranges {
                if let Some((converted_input_range, mut unconverted_input_ranges)) =
                    map.intersect(input_range)
                {
                    output.push(converted_input_range);
                    remains.append(&mut unconverted_input_ranges);
                } else {
                    remains.push(input_range.clone());
                }
            }
            input_ranges = remains.clone();
        }
        output.append(&mut remains);
        output
    }
}
