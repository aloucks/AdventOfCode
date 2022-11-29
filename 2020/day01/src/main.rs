use std::env;
use utility::format_err;
use utility::get_file_as_vec_string;
use utility::Result;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(format_err!(
            "need to pass input text path as first argument"
        ));
    }

    let contents = get_file_as_vec_string(&args[1])?;

    let x: Vec<i32> = contents
        .iter()
        .map(|l| l.parse::<i32>().unwrap_or_default())
        .collect();

    part_a(&x);
    part_b(&x);

    Ok(())
}

fn part_a(input: &[i32]) {
    'outer: for idx_1 in 0..input.len() {
        let val_1 = &input[idx_1];
        for (idx_2, val_2) in input.iter().enumerate() {
            if idx_1 == idx_2 {
                continue;
            }

            if val_1 + val_2 == 2020 {
                println!("{}/{}", idx_1, idx_2);
                println!("{}", val_1 * val_2);
                break 'outer;
            }
        }
    }
}

fn part_b(input: &[i32]) {
    'outer: for (idx_1, val_1) in input.iter().enumerate() {
        for (idx_2, val_2) in input.iter().enumerate() {
            if idx_2 == idx_1 {
                continue;
            }

            for (idx_3, val_3) in input.iter().enumerate() {
                if idx_1 == idx_3 || idx_2 == idx_3 {
                    continue;
                }

                if val_1 + val_2 + val_3 == 2020 {
                    println!("{}/{}/{}", idx_1, idx_2, idx_3);
                    println!("{}", val_1 * val_2 * val_3);
                    break 'outer;
                }
            }
        }
    }
}
