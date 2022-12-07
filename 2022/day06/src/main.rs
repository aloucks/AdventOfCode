use itertools::Itertools;
use std::fs;

fn main() -> utility::Result<()> {
    let input_path = utility::get_input_path()?;
    let input_data = fs::read_to_string(&input_path)?;

    parse(&input_data, 4);
    parse(&input_data, 14);
    Ok(())
}

fn parse(input: &str, width: usize) {
    input.lines().enumerate().for_each(|(i, l)| {
        let chars_to_start = parse_line(l, width);
        println!(
            "Line: {}, Width: {}, Chars to Start: {}",
            i, width, chars_to_start
        );
    });
}

fn parse_line(line: &str, width: usize) -> usize {
    for idx in 0..line.len() {
        let marker = &line[idx..idx + width];
        if marker.chars().all_unique() {
            return idx + width;
        }
    }
    0
}
