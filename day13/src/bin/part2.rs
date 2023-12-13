use color_eyre::Result;
use day13::{find_reflection, find_reflection_alt, read_input, to_array, to_u32};
use ndarray::Array2;

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;
    let patterns = contents.split("\n\n");

    let original_patterns = patterns
        .map(|s| s.trim())
        .map(to_array)
        .collect::<Result<Vec<Array2<u8>>>>()?;

    let mut total = 0;
    for original_pattern in original_patterns {
        let (_, y) = original_pattern.dim();
        let mut transposed_pattern = original_pattern.clone();
        transposed_pattern.swap_axes(0, 1);

        //for initial horizontal reflection
        let o = original_pattern
            .rows()
            .into_iter()
            .map(|row| to_u32(&row.to_vec()))
            .collect::<Vec<u32>>();
        let initial_horizontal_reflection = find_reflection(&o).unwrap_or(0);

        //for initial vertical reflection
        let t = transposed_pattern
            .rows()
            .into_iter()
            .map(|row| to_u32(&row.to_vec()))
            .collect::<Vec<u32>>();
        let initial_vertical_reflection = find_reflection(&t).unwrap_or(0);

        let mut current_horizontal_reflection = 0;
        let mut current_vertical_reflection = 0;
        //looking for smudges
        for (idx, elem) in original_pattern.iter().enumerate() {
            if *elem == 1 {
                continue;
            }
            let idx_x = idx / y;
            let idx_y = idx % y;
            let mut original = original_pattern.clone();
            *original.get_mut((idx_x, idx_y)).unwrap() = 1;
            let mut transposed = original.clone();
            transposed.swap_axes(0, 1);

            //for horizontal reflection
            let o = original
                .rows()
                .into_iter()
                .map(|row| to_u32(&row.to_vec()))
                .collect::<Vec<u32>>();
            current_horizontal_reflection += find_reflection_alt(&o)
                .map(|reflections| {
                    reflections
                        .iter()
                        .filter(|reflection| **reflection != initial_horizontal_reflection)
                        .sum::<usize>()
                })
                .unwrap_or(0);

            //for vertical reflection
            let t = transposed
                .rows()
                .into_iter()
                .map(|row| to_u32(&row.to_vec()))
                .collect::<Vec<u32>>();
            current_vertical_reflection += find_reflection_alt(&t)
                .map(|reflections| {
                    reflections
                        .iter()
                        .filter(|reflection| **reflection != initial_vertical_reflection)
                        .sum::<usize>()
                })
                .unwrap_or(0);
        }
        total += current_horizontal_reflection * 100 + current_vertical_reflection;
    }

    println!("{}", total);

    Ok(())
}
