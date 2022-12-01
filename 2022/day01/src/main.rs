use std::{env, fs};
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

    let mut result = count_calories(&input);

    result.sort();

    let max = result.iter().max();
    println!("{:?}", max);

    let r_len = result.len();

    let top_3 = result[r_len - 1] + result[r_len - 2] + result[r_len - 3];
    println!("{:?}", result);

    println!("{}", top_3);

    Ok(())
}

fn count_calories(input: &str) -> Vec<u32> {
    let mut result_1: Vec<Vec<u32>> = vec![];
    let mut inter: Vec<u32> = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            if inter.is_empty() {
                continue;
            }

            result_1.push(inter.clone());
            inter = vec![];
            continue;
        }

        inter.push(line.parse().unwrap());
    }

    // push remaining data
    if !inter.is_empty() {
        result_1.push(inter.clone());
    }

    result_1
        .iter()
        .map(|e| e.iter().sum::<u32>())
        .collect::<Vec<u32>>()
}
