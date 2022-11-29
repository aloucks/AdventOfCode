use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(anyhow::anyhow!(
            "need to pass input text path as first argument"
        ));
    }

    let input_file = File::open(&args[1])?;
    let reader = BufReader::new(input_file);

    let x: Vec<i32> = reader
        .lines()
        .map(|l| l.unwrap_or_default().parse::<i32>().unwrap_or_default())
        .collect();

    part_a(&x);
    part_b(&x);

    Ok(())
}

fn part_a(input: &Vec<i32>) {
    'outer: for idx_1 in 0..input.len() {
        let val_1 = &input[idx_1];
        for idx_2 in 0..input.len() {
            if idx_1 == idx_2 {
                continue;
            }

            let val_2 = &input[idx_2];

            if val_1 + val_2 == 2020 {
                println!("{}/{}", idx_1, idx_2);
                println!("{}", val_1 * val_2);
                break 'outer;
            }
        }
    }
}

fn part_b(input: &Vec<i32>) {
    'outer: for idx_1 in 0..input.len() {
        let val_1 = &input[idx_1];

        for idx_2 in 0..input.len() {
            if idx_2 == idx_1 {
                continue;
            }

            let val_2 = &input[idx_2];

            for idx_3 in 0..input.len() {
                if idx_1 == idx_3 || idx_2 == idx_3 {
                    continue;
                }

                let val_3 = &input[idx_3];

                if val_1 + val_2 + val_3 == 2020 {
                    println!("{}/{}/{}", idx_1, idx_2, idx_3);
                    println!("{}", val_1 * val_2 * val_3);
                    break 'outer;
                }
            }
        }
    }
}
