use std::env;
use utility::{format_err, get_file_as_vec_string, Result};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format_err!(
            "need to pass input text path as first argument"
        ));
    }

    let contents = get_file_as_vec_string(&args[1])?;

    let result = contents
        .iter()
        .map(|l| check_password_1(l).unwrap_or(false))
        .filter(|l| *l)
        .count();

    println!("{}", result);

    let result = contents
        .iter()
        .map(|l| check_password_2(l).unwrap_or(false))
        .filter(|l| *l)
        .count();

    println!("{}", result);

    Ok(())
}

fn check_password_1(input: &str) -> Result<bool> {
    let parts: Vec<&str> = input.split_ascii_whitespace().collect();

    if parts.len() != 3 {
        return Err(format_err!("must be 3 parts, got {}", parts.len()));
    }

    let counts: Vec<u32> = parts[0]
        .split('-')
        .map(|n| n.parse::<u32>().unwrap_or_default())
        .collect();

    if counts.len() != 2 {
        return Err(format_err!("must be 2 counts, got {}", counts.len()));
    }

    let letter = parts[1].replace(':', "");

    if letter.len() != 1 {
        return Err(format_err!("must be 1 letter, got {}", letter.len()));
    }

    let check = parts[2].matches(&letter).count() as u32;

    let result = check >= counts[0] && check <= counts[1];

    Ok(result)
}

fn check_password_2(input: &str) -> Result<bool> {
    let parts: Vec<&str> = input.split_ascii_whitespace().collect();

    if parts.len() != 3 {
        return Err(format_err!("must be 3 parts, got {}", parts.len()));
    }

    let counts: Vec<u32> = parts[0]
        .split('-')
        .map(|n| n.parse::<u32>().unwrap_or_default())
        .collect();

    if counts.len() != 2 {
        return Err(format_err!("must be 2 counts, got {}", counts.len()));
    }

    if parts[2].len() < counts[1].try_into().unwrap() {
        return Err(format_err!(
            "string too short {} {}",
            parts[2].len(),
            counts[1]
        ));
    }

    if counts[0] < 1 || counts[1] < 1 {
        return Err(format_err!(
            "cannot have 0 indexing, all indexing starts at 1"
        ));
    }

    let letter = parts[1].replace(':', "");

    if letter.len() != 1 {
        return Err(format_err!("must be 1 letter, got {}", letter.len()));
    }

    let letter_char = letter.chars().nth(0).unwrap();

    let idx_1: usize = (counts[0] - 1).try_into()?;
    let idx_2: usize = (counts[1] - 1).try_into()?;

    let p_1 = parts[2].chars().nth(idx_1).unwrap();
    let p_2 = parts[2].chars().nth(idx_2).unwrap();

    let result =
        (p_1 == letter_char && p_2 != letter_char) || (p_1 != letter_char && p_2 == letter_char);

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::{check_password_1, check_password_2};

    #[test]
    fn example_1_1_is_true() {
        let result = check_password_1("1-3 a: abcde").unwrap();
        assert_eq!(true, result);
    }

    #[test]
    fn example_1_2_is_false() {
        let result = check_password_1("1-3 b: cdefg").unwrap();
        assert_eq!(false, result);
    }

    #[test]
    fn example_1_3_is_true() {
        let result = check_password_1("2-9 c: ccccccccc").unwrap();
        assert_eq!(true, result);
    }

    #[test]
    fn example_2_1_is_true() {
        let result = check_password_2("1-3 a: abcde").unwrap();
        assert_eq!(true, result);
    }

    #[test]
    fn example_2_2_is_false() {
        let result = check_password_2("1-3 b: cdefg").unwrap();
        assert_eq!(false, result);
    }

    #[test]
    fn example_2_3_is_false() {
        let result = check_password_2("2-9 c: ccccccccc").unwrap();
        assert_eq!(false, result);
    }
}
