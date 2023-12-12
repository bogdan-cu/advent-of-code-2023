use color_eyre::eyre::{eyre, Ok, Result};
use day11::{get_distance, read_input};

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;
    let mut map: Vec<Vec<usize>> = Vec::new();
    let x = contents.lines().count();
    let y = contents
        .lines()
        .map(|line| line.len())
        .max()
        .ok_or(eyre!("empty input"))?;
    let mut galaxy_counter = 0_usize;
    for line in contents.lines() {
        let row: Result<Vec<usize>> = line
            .chars()
            .map(|chr| match chr {
                '.' => Ok(0),
                '#' => {
                    galaxy_counter += 1;
                    Ok(galaxy_counter)
                }
                x => Err(eyre!("found odd character in input: {}", x)),
            })
            .collect();
        map.push(row?);
    }

    let mut line_expansions = Vec::<usize>::new();
    let mut expansions = 0_usize;
    for (idx, line) in map.iter().enumerate() {
        if line.iter().sum::<usize>() == 0 {
            expansions += 1;
            line_expansions.push(idx + expansions);
        };
    }
    let expansion_line = vec![0; y];
    for expansion_idx in line_expansions {
        map.insert(expansion_idx, expansion_line.clone());
    }

    let mut column_expansions = Vec::<usize>::new();
    let mut expansions = 0_usize;
    for idx in 0..y {
        if map.iter().map(|row| *row.get(idx).unwrap()).sum::<usize>() == 0 {
            expansions += 1;
            column_expansions.push(idx + expansions);
        }
    }
    let map_final: Vec<Vec<usize>> = map
        .into_iter()
        .map(|mut row| {
            for expansion_idx in column_expansions.iter() {
                row.insert(*expansion_idx, 0);
            }
            row
        })
        .collect();

    let galaxies: Vec<(usize, usize)> = map_final
        .iter()
        .enumerate()
        .flat_map(|(idx_x, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, elem)| **elem > 0)
                .map(move |(idx_y, _)| (idx_x, idx_y))
        })
        .collect();
    let mut total = 0;
    for i in 0..galaxy_counter - 1 {
        let fst = galaxies.get(i).unwrap();
        let lst = galaxies.get(i + 1..galaxy_counter).unwrap();
        total += lst
            .iter()
            .map(|galaxy| get_distance(fst, galaxy))
            .sum::<isize>();
    }

    println!("{}", total);

    Ok(())
}
