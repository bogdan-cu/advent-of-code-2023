use day03::{parse_alone, parse_with_one_adjacent_line, parse_with_two_adjacent_lines, read_input};

pub fn main() -> color_eyre::Result<()> {
    let contents = read_input("./src/bin/input.txt")?;
    let lines: Vec<&str> = contents.lines().collect();
    let line_count = lines.len();

    let mut sum = 0;
    let mut idx = 0;
    while idx < line_count {
        if idx > 0 && idx < line_count - 1 {
            sum += parse_with_two_adjacent_lines(lines[idx], lines[idx - 1], lines[idx + 1]);
            idx += 1;
            continue;
        }
        if idx < line_count - 1 {
            sum += parse_with_one_adjacent_line(lines[idx], lines[idx + 1]);
            idx += 1;
            continue;
        }
        if idx > 0 {
            sum += parse_with_one_adjacent_line(lines[idx], lines[idx - 1]);
            idx += 1;
            continue;
        }
        sum += parse_alone(lines[idx]);
        idx += 1;
    }

    println!("total is {}", sum);

    Ok(())
}
