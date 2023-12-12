use std::collections::VecDeque;

use color_eyre::eyre::{eyre, Ok, Result};
use day10::{read_input, Direction, Pipe};
use ndarray::Array2;

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;
    let lines: Vec<Vec<char>> = contents
        .lines()
        .map(|s| {
            let v: Vec<char> = s.chars().collect();
            v
        })
        .collect();
    let x = lines.len();
    let y = lines
        .iter()
        .map(|line| line.len())
        .max()
        .ok_or(eyre!("empty line detected"))?;

    let mut pipe_arr = Array2::<Pipe>::default((x, y));
    for (idx_x, val_x) in lines.iter().enumerate() {
        for (idx_y, val_y) in val_x.iter().enumerate() {
            *pipe_arr.get_mut((idx_x, idx_y)).unwrap() = Pipe::new(val_y)?;
        }
    }
    let start_pos = pipe_arr
        .iter()
        .position(|elem| elem.is_source())
        .ok_or(eyre!("could not find starting position"))?;
    let start_pos = (start_pos / y, start_pos % y);

    let mut distance_arr = Array2::<isize>::from_elem((x, y), -2);

    let mut queue = VecDeque::<(usize, usize)>::new();
    queue.push_back(start_pos);
    let mut distance = 0_isize;
    *distance_arr.get_mut(start_pos).unwrap() = distance;
    let directions = [
        Direction::East,
        Direction::North,
        Direction::West,
        Direction::South,
    ];

    while !queue.is_empty() {
        let mut neighbours = Vec::<(usize, usize)>::new();
        while !queue.is_empty() {
            let idx = queue.pop_front().unwrap();
            let elem = pipe_arr.get(idx).unwrap();
            *distance_arr.get_mut(idx).unwrap() = distance;

            for direction in directions.iter() {
                let neighbour_idx = match direction.from_pos(idx, 1) {
                    Some(val) => val,
                    None => continue,
                };
                if let Some(neighbour) = pipe_arr.get(neighbour_idx) {
                    if *distance_arr.get(neighbour_idx).unwrap() < 0
                        && !neighbour.is_ground()
                        && elem.is_connected_to(neighbour, idx, neighbour_idx)
                        && !neighbours.iter().any(|elem| *elem == neighbour_idx)
                    {
                        neighbours.push(neighbour_idx);
                    }
                }
            }
        }
        for neighbour in neighbours {
            queue.push_back(neighbour);
        }
        distance += 1;
    }

    let mut new_distance_arr = Array2::<isize>::from_elem((x * 3, y * 3), -2);
    for (idx, elem) in distance_arr.iter().enumerate() {
        let idx = (idx / y, idx % y);
        if *elem != -2 {
            *new_distance_arr
                .get_mut((idx.0 * 3 + 1, idx.1 * 3 + 1))
                .unwrap() = *elem;
        }
    }

    for (idx, elem) in distance_arr.iter().enumerate() {
        let idx = (idx / y, idx % y);
        for direction in directions.iter() {
            let neighbour = match direction
                .from_pos(idx, 1)
                .and_then(|position| distance_arr.get(position))
                .filter(|value| **value >= 0)
            {
                Some(val) => val,
                None => continue,
            };
            if (*neighbour - *elem).abs() == 1 {
                *new_distance_arr
                    .get_mut(
                        direction
                            .from_pos((3 * idx.0 + 1, 3 * idx.1 + 1), 1)
                            .unwrap(),
                    )
                    .unwrap() = 1;
                *new_distance_arr
                    .get_mut(
                        direction
                            .from_pos((3 * idx.0 + 1, 3 * idx.1 + 1), 2)
                            .unwrap(),
                    )
                    .unwrap() = 1;
            }
        }
    }

    let map_arr = new_distance_arr.map(|elem| if *elem >= 0 { 1 } else { 0 });
    let mut count = 0;
    let map_x = x * 3;
    let map_y = y * 3;
    let mut elems = Vec::<i32>::new();

    for idx_x in 0..x {
        for idx_y in 0..y {
            let new_idx_x = 3 * idx_x + 1;
            let new_idx_y = 3 * idx_y + 1;
            let new_idx = (new_idx_x, new_idx_y);
            if *map_arr.get(new_idx).unwrap() == 1 {
                continue;
            }

            let mut is_open = false;
            for y in 0..map_y {
                elems.clear();
                if is_open {
                    break;
                }
                for j in y..=new_idx_y {
                    elems.push(*map_arr.get((new_idx_x, j)).unwrap());
                }
                for j in new_idx_y..=y {
                    elems.push(*map_arr.get((new_idx_x, j)).unwrap());
                }
                if elems.iter().any(|elem| *elem == 1) {
                    continue;
                }
                elems.clear();
                for i in 0..=new_idx_x {
                    elems.push(*map_arr.get((i, y)).unwrap());
                }
                if elems.iter().all(|elem| *elem == 0) {
                    is_open = true;
                    break;
                }
                elems.clear();
                for i in new_idx_x..map_x {
                    elems.push(*map_arr.get((i, y)).unwrap());
                }
                if elems.iter().all(|elem| *elem == 0) {
                    is_open = true;
                    break;
                }
                elems.clear();
            }

            for x in 0..map_x {
                elems.clear();
                if is_open {
                    break;
                }
                for j in x..=new_idx_x {
                    elems.push(*map_arr.get((j, new_idx_y)).unwrap());
                }
                for j in new_idx_x..=x {
                    elems.push(*map_arr.get((j, new_idx_y)).unwrap());
                }
                if elems.iter().any(|elem| *elem == 1) {
                    continue;
                }
                for i in 0..=new_idx_y {
                    elems.push(*map_arr.get((x, i)).unwrap());
                }
                if elems.iter().all(|elem| *elem == 0) {
                    is_open = true;
                    break;
                }
                elems.clear();
                for i in new_idx_y..map_y {
                    elems.push(*map_arr.get((x, i)).unwrap());
                }
                if elems.iter().all(|elem| *elem == 0) {
                    is_open = true;
                    break;
                }
                elems.clear();
            }

            if !is_open {
                count += 1;
            }
        }
    }

    println!("{}", count);

    Ok(())
}
