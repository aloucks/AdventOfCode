use std::fs;
use utility::get_input_path;
use utility::Result;

fn main() -> Result<()> {
    let input_file = get_input_path()?;

    let input = fs::read_to_string(input_file)?;

    let result = count_calories(&input);

    println!("Max Calories: {}", result[0]);

    let top_three = (&result[0..3]).iter().sum::<u32>();

    println!("Top 3 Max Calories: {}", top_three);

    Ok(())
}

fn count_calories(input: &str) -> Vec<u32> {
    let elf_delim = if input.contains("\r\n\r\n") {
        "\r\n\r\n"
    } else {
        "\n\n"
    };

    let snack_delim = if input.contains("\r\n") { "\r\n" } else { "\n" };

    let mut calories: Vec<u32> = input
        .trim()
        .split(elf_delim)
        .into_iter()
        .map(|e| {
            e.split(snack_delim)
                .map(|s| s.parse::<u32>().unwrap())
                .sum()
        })
        .collect();

    calories.sort_by(|a, b| b.cmp(a));

    calories
}
