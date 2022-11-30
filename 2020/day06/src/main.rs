use std::{collections::HashSet, env, fs};
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

    let result = collect_customs_data_a(&input);
    println!("{}", result);

    let result = collect_customs_data_b(&input);
    println!("{}", result);

    Ok(())
}

fn collect_customs_data_a(input: &str) -> usize {
    let mut result_1: Vec<Vec<String>> = vec![];
    let mut inter: Vec<String> = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            if inter.is_empty() {
                continue;
            }

            result_1.push(inter.clone());
            inter = vec![];
            continue;
        }

        inter.push(line.to_string());
    }

    // push remaining data
    if !inter.is_empty() {
        result_1.push(inter.clone());
    }

    let result_2: Vec<String> = result_1.iter().map(|c| c.join("")).collect();

    let result_3: Vec<HashSet<char>> = result_2
        .iter()
        .map(|l| {
            let mut chars: HashSet<char> = HashSet::new();

            l.chars().for_each(|c| {
                chars.insert(c);
            });
            chars
        })
        .collect();

    let mut sum = 0;
    result_3.iter().for_each(|s| {
        sum += s.len();
    });

    sum
}

fn collect_customs_data_b(input: &str) -> i32 {
    let mut result_1: Vec<Vec<String>> = vec![];
    let mut inter: Vec<String> = vec![];

    for line in input.lines() {
        if line.trim().is_empty() {
            if inter.is_empty() {
                continue;
            }

            result_1.push(inter.clone());
            inter = vec![];
            continue;
        }

        inter.push(line.to_string());
    }

    // push remaining data
    if !inter.is_empty() {
        result_1.push(inter.clone());
    }

    let result_2: Vec<(String, usize)> = result_1.iter().map(|c| (c.join(""), c.len())).collect();

    let qs = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let result_3: Vec<i32> = result_2
        .iter()
        .map(|l| {
            let mut count = 0;

            qs.iter().for_each(|c| {
                let matches = l.0.matches(*c).count();

                if matches == l.1 {
                    count += 1;
                }
            });

            count
        })
        .collect();

    result_3.iter().sum()
}
