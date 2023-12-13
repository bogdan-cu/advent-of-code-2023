use color_eyre::Result;
use day13::{find_reflection_alt2, read_input, to_array, to_u32};
use ndarray::Array2;

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;
    let patterns = contents.split("\n\n");

    let original_patterns = patterns
        .map(|s| s.trim())
        .map(to_array)
        .collect::<Result<Vec<Array2<u8>>>>()?;
    let transposed_patterns = original_patterns
        .clone()
        .into_iter()
        .map(|mut pattern| {
            pattern.swap_axes(0, 1);
            pattern
        })
        .collect::<Vec<Array2<u8>>>();
    let patterns = original_patterns
        .into_iter()
        .zip(transposed_patterns)
        .collect::<Vec<(Array2<u8>, Array2<u8>)>>();

    let mut total = 0;
    for (original, transposed) in patterns {
        //for horizontal reflection
        let o = original
            .rows()
            .into_iter()
            .map(|row| to_u32(&row.to_vec()))
            .collect::<Vec<u32>>();
        total += find_reflection_alt2(&o).unwrap_or(0) * 100;

        //for vertical reflection
        let t = transposed
            .rows()
            .into_iter()
            .map(|row| to_u32(&row.to_vec()))
            .collect::<Vec<u32>>();
        total += find_reflection_alt2(&t).unwrap_or(0);
    }

    println!("{}", total);

    Ok(())
}
