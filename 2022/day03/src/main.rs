fn main() -> utility::Result<()> {
    let input_file = utility::get_input_path()?;
    let input_contents = utility::get_file_as_vec_string(&input_file)?;

    let priority_sum: &u32 = &input_contents.iter().map(|l| find_priority(l)).sum();

    println!("{}", priority_sum);

    let priority_sum: u32 = input_contents.chunks(3).map(|c| find_priority_2(c)).sum();

    println!("{}", priority_sum);

    Ok(())
}

fn find_priority(input: &str) -> u32 {
    let a = &input[0..input.len() / 2];
    let b = &input[input.len() / 2..input.len()];

    let mut item = '-';

    for c in a.chars() {
        if b.contains(c) {
            item = c;
            break;
        }
    }

    get_priority(item) as u32
}

fn find_priority_2(input: &[String]) -> u32 {
    let mut item: char = '_';

    for c in input[0].chars() {
        if input[1].contains(c) && input[2].contains(c) {
            item = c;
            break;
        }
    }

    get_priority(item) as u32
}

fn get_priority(item: char) -> u32 {
    let item = item as u8;

    match item {
        result if (b'a'..=b'z').contains(&item) => (result - b'a' + 1).into(),
        result if (b'A'..=b'Z').contains(&item) => (result - b'A' + 27).into(),
        _ => panic!("must be a-z or A-Z only"),
    }
}
