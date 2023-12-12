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

    let mut farthest_element = 0_isize;
    for (idx, distance) in distance_arr.iter().enumerate() {
        let idx = (idx / y, idx % y);
        let connected_neighbours: Vec<&isize> = directions
            .iter()
            .flat_map(|direction| direction.from_pos(idx, 1))
            .flat_map(|position| distance_arr.get(position))
            .filter(|elem| **elem == *distance - 1)
            .collect();
        if connected_neighbours.len() == 2 && *distance > farthest_element {
            farthest_element = *distance;
        }
    }
    let map_arr = distance_arr.map(|elem| if *elem >= 0 { 1 } else { 0 });
    println!("{:?}", &map_arr);
    println!("result is {}", farthest_element);

    Ok(())
}
