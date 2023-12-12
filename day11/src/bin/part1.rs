use color_eyre::eyre::{eyre, Ok, Result};
use day11::{get_distance, read_input};

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;
    let mut map: Vec<Vec<usize>> = Vec::new();
    let _x = contents.lines().count();
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

    let line_expansions: Vec<usize> = map
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|elem| *elem == 0))
        .map(|(idx, _)| idx)
        .collect();

    let mut column_expansions = Vec::<usize>::new();
    for idx in 0..y {
        if map.iter().map(|row| *row.get(idx).unwrap()).sum::<usize>() == 0 {
            column_expansions.push(idx);
        }
    }

    let galaxies: Vec<(usize, usize)> = map
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
            .map(|galaxy| get_distance(fst, galaxy, &line_expansions, &column_expansions, 1))
            .sum::<usize>();
    }

    println!("{}", total);

    Ok(())
}
