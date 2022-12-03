const letters: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> utility::Result<()> {
    let input_file = utility::get_input_path()?;
    let input_contents = utility::get_file_as_vec_string(&input_file)?;

    let priority_sum: &u32 = &input_contents.iter().map(|l| find_priority(l)).sum();

    println!("{}", priority_sum);

    let mut priority_sum = 0;
    for (idx, _) in input_contents.iter().enumerate().step_by(3) {
        let end = idx + 3;
        priority_sum += find_priority_2(&input_contents[idx..end]);
    }

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

    (letters.find(item).unwrap() + 1) as u32
}

fn find_priority_2(input: &[String]) -> u32 {
    println!("{:?}", input);
    let mut item: char = '_';

    for c in input[0].chars() {
        if input[1].contains(c) && input[2].contains(c) {
            item = c;
            break;
        }
    }

    println!("{}", item);

    (letters.find(item).unwrap() + 1) as u32
}
