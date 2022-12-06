use std::fs;

use itertools::Itertools;

fn main() -> utility::Result<()> {
    let input_path = utility::get_input_path()?;
    let input_data = fs::read_to_string(&input_path)?;

    parse_1(&input_data);
    parse_2(&input_data);
    Ok(())
}

fn parse_1(input: &str) {
    input.lines().for_each(|l| {
        let r = parse_line_1(l);
        println!("{}", r);
    })
}

fn parse_line_1(line: &str) -> usize {
    for idx in 0..line.len() {
        let marker = &line[idx..idx + 4];
        if check_marker_dupe_1(marker) {
            return idx + 4;
        }
    }
    0
}

fn check_marker_dupe_1(marker: &str) -> bool {
    marker.chars().sorted().dedup().count() == 4
}

fn parse_2(input: &str) {
    input.lines().for_each(|l| {
        let r = parse_line_2(l);
        println!("{}", r);
    })
}

fn parse_line_2(line: &str) -> usize {
    for idx in 0..line.len() {
        let marker = &line[idx..idx + 14];
        if check_marker_dupe_2(marker) {
            return idx + 14;
        }
    }
    0
}

fn check_marker_dupe_2(marker: &str) -> bool {
    marker.chars().sorted().dedup().count() == 14
}
