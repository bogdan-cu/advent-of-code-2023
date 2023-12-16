use color_eyre::eyre::Result;
use day14::{read_input, to_array};

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;
    let platform = to_array(&contents)?;
    let mut transposed_platform = platform.clone();
    transposed_platform.swap_axes(0, 1);

    let result = transposed_platform
        .rows()
        .into_iter()
        .map(|row| {
            let mut line = row.to_vec();
            let cubes = line
                .iter()
                .enumerate()
                .filter(|(_, elem)| **elem == '#')
                .map(|(idx, _)| idx as isize)
                .collect::<Vec<isize>>();

            let mut slide_sec_start_pos = cubes.iter().map(|idx| idx + 1).collect::<Vec<isize>>();
            slide_sec_start_pos.insert(0, 0);
            let mut slide_sec_end_pos = cubes.iter().map(|idx| idx - 1).collect::<Vec<isize>>();
            slide_sec_end_pos.push(line.len() as isize - 1);

            let slide_sections = slide_sec_start_pos
                .into_iter()
                .zip(slide_sec_end_pos)
                .filter(|(start, end)| *start >= 0 || *end >= 0)
                .map(|(start, end)| (start as usize, end as usize))
                .filter(|(start, end)| line.get(*start..=*end).is_some())
                .collect::<Vec<(usize, usize)>>();

            for (start, end) in slide_sections {
                let section = line.get_mut(start..=end).unwrap();
                let smooth_stones = section
                    .iter()
                    .enumerate()
                    .filter(|(_, elem)| **elem == 'O')
                    .count();
                for (idx, elem) in section.iter_mut().enumerate() {
                    if idx < smooth_stones {
                        *elem = 'O';
                    } else {
                        *elem = '.';
                    }
                }
            }

            line.iter()
                .enumerate()
                .filter(|(_, elem)| **elem == 'O')
                .map(|(idx, _)| line.len() - idx)
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("result is {}", result);

    Ok(())
}
