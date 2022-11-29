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

    let mut count = 0;

    for ppd in &passport_data {
        if validate_passport_data_b(ppd) {
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

fn validate_passport_data_b(input: &str) -> bool {
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

    for piece in &pieces {
        let kvp: Vec<&str> = piece.splitn(2, ":").collect();

        let result = match kvp[0] {
            "byr" => validate_byr(kvp[1]),
            "ecl" => validate_ecl(kvp[1]),
            "eyr" => validate_eyr(kvp[1]),
            "hcl" => validate_hcl(kvp[1]),
            "hgt" => validate_hgt(kvp[1]),
            "iyr" => validate_iyr(kvp[1]),
            "pid" => validate_pid(kvp[1]),
            _ => true,
        };

        if !result {
            return false;
        }
    }

    true
}

fn validate_byr(input: &str) -> bool {
    let birth_year: u32 = input.parse().unwrap();
    birth_year >= 1920 && birth_year <= 2002
}

fn validate_ecl(input: &str) -> bool {
    let count = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .filter(|x| x.eq(&&input))
        .count();

    count > 0
}

fn validate_eyr(input: &str) -> bool {
    let expire_year: u32 = input.parse().unwrap();

    expire_year >= 2020 && expire_year <= 2030
}

fn validate_hcl(input: &str) -> bool {
    if !input.starts_with("#") {
        return false;
    }

    let check = input.trim_start_matches("#");

    if check.len() != 6 {
        return false;
    }

    check.chars().all(char::is_alphanumeric)
}

fn validate_hgt(input: &str) -> bool {
    if input.ends_with("cm") {
        let in_cm: u32 = input.trim_end_matches("cm").parse().unwrap();

        return in_cm >= 150 && in_cm <= 193;
    }

    if input.ends_with("in") {
        let in_in: u32 = input.trim_end_matches("in").parse().unwrap();

        return in_in >= 59 && in_in <= 76;
    }

    false
}

fn validate_iyr(input: &str) -> bool {
    let issue_year: u32 = input.parse().unwrap();

    issue_year >= 2010 && issue_year <= 2020
}

fn validate_pid(input: &str) -> bool {
    input.len() == 9 && input.chars().all(char::is_numeric)
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
