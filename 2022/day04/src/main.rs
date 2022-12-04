use std::fs;

fn main() -> utility::Result<()> {
    let input_path = utility::get_input_path()?;
    let input_data = fs::read_to_string(&input_path)?;

    parse_1(&input_data);

    Ok(())
}

struct Section {
    start: usize,
    end: usize,
}

impl Section {
    fn parse(start: &str, end: &str) -> Section {
        Section {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        }
    }
}

fn parse_1(input_data: &str) {
    let ranges = input_data
        .lines()
        .map(|l| {
            let l_split = l.split_once(',').unwrap();
            let l_range = l_split.0.split_once('-').unwrap();
            let r_range = l_split.1.split_once('-').unwrap();

            (
                Section::parse(l_range.0, l_range.1),
                Section::parse(r_range.0, r_range.1),
            )
        })
        .collect::<Vec<_>>();

    let mut count_1 = 0;
    let mut count_2 = 0;

    for range in ranges {
        let check_1 = contains(&range);

        if check_1 {
            count_1 += 1;
        }

        let check_2 = overlap(&range);

        if check_2 {
            count_2 += 1;
        }
    }

    println!("{}", count_1);
    println!("{}", count_2);
}

fn contains(input: &(Section, Section)) -> bool {
    (input.0.start >= input.1.start && input.0.end <= input.1.end)
        || (input.1.start >= input.0.start && input.1.end <= input.0.end)
}

fn overlap(input: &(Section, Section)) -> bool {
    if input.0.start <= input.1.start && input.0.end >= input.1.start {
        return true;
    }

    if input.0.start <= input.1.end && input.0.end >= input.1.end {
        return true;
    }

    if input.1.start <= input.0.start && input.1.end >= input.0.start {
        return true;
    }

    if input.1.start <= input.0.end && input.1.end >= input.0.end {
        return true;
    }

    false
}
