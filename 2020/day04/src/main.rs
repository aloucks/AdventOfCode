use std::env;
use std::fs;
use utility::format_err;
use utility::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format_err!(
            "need to pass input text path as first argument"
        ));
    }

    let input = fs::read_to_string(&args[1])?;

    let passport_data = collect_passport_data(&input);

    let mut count = 0;

    for ppd in &passport_data {
        if validate_passport_data_a(ppd) {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(())
}

fn collect_passport_data(input: &str) -> Vec<String> {
    let mut result_1: Vec<Vec<String>> = vec![];
    let mut inter: Vec<String> = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            if inter.is_empty() {
                continue;
            }

            result_1.push(inter.clone());
            inter = vec![];
        }

        inter.push(line.to_string());
    }

    // push remaining data
    if !inter.is_empty() {
        result_1.push(inter.clone());
    }

    result_1
        .iter()
        .map(|j| j.join(" "))
        .collect::<Vec<String>>()
}

fn validate_passport_data_a(input: &str) -> bool {
    // cid left out
    let valid_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let pieces = input.split_ascii_whitespace().collect::<Vec<&str>>();

    // need at least this many keys
    if pieces.len() < valid_keys.len() {
        return false;
    }

    for valid_key in &valid_keys {
        let mut found = false;
        for piece in &pieces {
            if piece.starts_with(valid_key) {
                found = true;
                break;
            }
        }

        if !found {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::collect_passport_data;
    use crate::validate_passport_data_a;

    pub const TEST_DATA: &str = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

    #[test]
    fn collected_passport_data_should_be_len_4() {
        let result = collect_passport_data(TEST_DATA);

        assert_eq!(4, result.len());
    }

    #[test]
    fn validate_passport_data_a_should_work() {
        let result = collect_passport_data(TEST_DATA);

        assert_eq!(4, result.len());

        let r_0 = validate_passport_data_a(&result[0]);
        assert_eq!(true, r_0);

        let r_1 = validate_passport_data_a(&result[1]);
        assert_eq!(false, r_1);

        let r_2 = validate_passport_data_a(&result[2]);
        assert_eq!(true, r_2);

        let r_3 = validate_passport_data_a(&result[3]);
        assert_eq!(false, r_3);
    }
}
