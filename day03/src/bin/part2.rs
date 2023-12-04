use day03::{contains_wheel, parse_line, read_input};

pub fn main() -> color_eyre::Result<()> {
    let contents = read_input("./src/bin/input.txt")?;
    let lines: Vec<&str> = contents.lines().collect();
    let line_count = lines.len();

    let mut idx = 0;
    let mut sum_ratios = 0;
    while idx < line_count {
        let line = lines.get(idx).unwrap();
        if let Some(positions) = contains_wheel(line) {
            for wheel_position in positions {
                let mut line_parts = parse_line(line, wheel_position);
                if let Some(prev_line) = lines.get(idx - 1) {
                    line_parts.append(&mut parse_line(prev_line, wheel_position));
                }
                if let Some(next_line) = lines.get(idx + 1) {
                    line_parts.append(&mut parse_line(next_line, wheel_position));
                }
                if line_parts.len() == 2 {
                    sum_ratios += line_parts.into_iter().product::<usize>();
                }
            }
            idx += 1;
        } else {
            idx += 1;
        }
    }

    println!("total is {}", sum_ratios);

    Ok(())
}
