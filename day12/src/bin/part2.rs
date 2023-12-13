use color_eyre::Result;
use day12::{
    generate_constraints_pt2, generate_working_spring_sections, parse_input_line, read_input,
    string_is_valid,
};

pub fn main() -> Result<()> {
    let contents = read_input("./src/input.txt")?;
    let parsed_input: Result<Vec<(&str, Vec<usize>)>> =
        contents.lines().map(parse_input_line).collect();
    let parsed_input = parsed_input?;

    let mut valid_string_counter = 0;
    for line in parsed_input.iter() {
        let initial_damaged_spring_list = line.1.clone();
        let mut damaged_spring_list = initial_damaged_spring_list.clone();
        for _ in 1..5 {
            damaged_spring_list.append(&mut initial_damaged_spring_list.clone());
        }
        let damaged_section_count = damaged_spring_list.len();

        let known_sections_generated: Vec<(String, usize)> = damaged_spring_list
            .iter()
            .map(|elem| {
                let r: String = (0..*elem).map(|_| '#').collect();
                let length = r.len();
                (r, length)
            })
            .collect();

        let mut damaged_sections: Vec<String> = known_sections_generated
            .iter()
            .map(|(section, _)| section.to_string())
            .collect();
        let damaged_sections_length = known_sections_generated
            .iter()
            .map(|(_, length)| *length)
            .sum::<usize>();

        let (spring_list_length, constraints) = generate_constraints_pt2(line.0);
        let remaining_length = spring_list_length - damaged_sections_length;

        let possible_new_sections_list =
            match generate_working_spring_sections(remaining_length, damaged_section_count) {
                Some(val) => val,
                None => continue,
            };
        for mut possible_new_sections in possible_new_sections_list.into_iter() {
            let new_section_count = possible_new_sections.len();

            if new_section_count >= damaged_section_count {
                damaged_sections.push("".to_string());
                let result = possible_new_sections
                    .iter()
                    .zip(damaged_sections.iter())
                    .map(|(fst, lst)| fst.to_string() + lst)
                    .fold("".to_string(), |acc, elem| acc + elem.as_str());

                let _ = damaged_sections.pop();
                valid_string_counter += string_is_valid(&result, &constraints) as usize;
            }

            if new_section_count <= damaged_section_count {
                possible_new_sections.push("".to_string());
                let result = damaged_sections
                    .iter()
                    .zip(possible_new_sections.iter())
                    .map(|(fst, lst)| fst.to_string() + lst)
                    .fold("".to_string(), |acc, elem| acc + elem.as_str());

                valid_string_counter += string_is_valid(&result, &constraints) as usize;
            }
        }
    }
    println!("{}", valid_string_counter);
    Ok(())
}
