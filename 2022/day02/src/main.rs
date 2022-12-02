fn main() -> utility::Result<()> {
    let input_file = utility::get_input_path()?;
    let input = utility::get_file_as_vec_string(&input_file)?;

    let input_1 = input
        .iter()
        .map(|l| line_to_rps_tuple(l))
        .collect::<Vec<(RPS, RPS)>>();

    let score: u32 = input_1.iter().map(rps_tuple_to_score).sum();

    println!("{:?}", score);

    let input_1 = input
        .iter()
        .map(|l| line_to_rps2_tuple(l))
        .collect::<Vec<(RPS2, RPS2)>>();

    let score: u32 = input_1
        .iter()
        .map(rps2_tuple_to_rps_tuple)
        .map(|l| rps_tuple_to_score(&l))
        .sum();

    println!("{:?}", score);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for RPS {
    fn from(c: char) -> Self {
        match c {
            'A' => RPS::Rock,
            'B' => RPS::Paper,
            'C' => RPS::Scissors,
            'X' => RPS::Rock,
            'Y' => RPS::Paper,
            'Z' => RPS::Scissors,
            unk => panic!("unknown value {}", unk),
        }
    }
}

fn line_to_rps_tuple(input: &str) -> (RPS, RPS) {
    let play: (RPS, RPS) = (
        input.chars().next().unwrap().into(),
        input.chars().last().unwrap().into(),
    );
    play
}

fn rps_tuple_to_score(input: &(RPS, RPS)) -> u32 {
    match input {
        (RPS::Rock, RPS::Rock) => 3 + rps_to_value(&RPS::Rock),
        (RPS::Rock, RPS::Paper) => 6 + rps_to_value(&RPS::Paper),
        (RPS::Rock, RPS::Scissors) => rps_to_value(&RPS::Scissors),
        (RPS::Paper, RPS::Rock) => rps_to_value(&RPS::Rock),
        (RPS::Paper, RPS::Paper) => 3 + rps_to_value(&RPS::Paper),
        (RPS::Paper, RPS::Scissors) => 6 + rps_to_value(&RPS::Scissors),
        (RPS::Scissors, RPS::Rock) => 6 + rps_to_value(&RPS::Rock),
        (RPS::Scissors, RPS::Paper) => rps_to_value(&RPS::Paper),
        (RPS::Scissors, RPS::Scissors) => 3 + rps_to_value(&RPS::Scissors),
    }
}

fn rps_to_value(input: &RPS) -> u32 {
    match input {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    }
}

#[derive(Debug, PartialEq)]
enum RPS2 {
    Rock,
    Paper,
    Scissors,
    Lose,
    Draw,
    Win,
}

impl From<char> for RPS2 {
    fn from(c: char) -> Self {
        match c {
            'A' => RPS2::Rock,
            'B' => RPS2::Paper,
            'C' => RPS2::Scissors,
            'X' => RPS2::Lose,
            'Y' => RPS2::Draw,
            'Z' => RPS2::Win,
            unk => panic!("unknown value {}", unk),
        }
    }
}

fn line_to_rps2_tuple(input: &str) -> (RPS2, RPS2) {
    let play: (RPS2, RPS2) = (
        input.chars().next().unwrap().into(),
        input.chars().last().unwrap().into(),
    );
    play
}

fn rps2_tuple_to_rps_tuple(input: &(RPS2, RPS2)) -> (RPS, RPS) {
    match input {
        (RPS2::Rock, RPS2::Lose) => (RPS::Rock, RPS::Scissors),
        (RPS2::Rock, RPS2::Draw) => (RPS::Rock, RPS::Rock),
        (RPS2::Rock, RPS2::Win) => (RPS::Rock, RPS::Paper),
        (RPS2::Paper, RPS2::Lose) => (RPS::Paper, RPS::Rock),
        (RPS2::Paper, RPS2::Draw) => (RPS::Paper, RPS::Paper),
        (RPS2::Paper, RPS2::Win) => (RPS::Paper, RPS::Scissors),
        (RPS2::Scissors, RPS2::Lose) => (RPS::Scissors, RPS::Paper),
        (RPS2::Scissors, RPS2::Draw) => (RPS::Scissors, RPS::Scissors),
        (RPS2::Scissors, RPS2::Win) => (RPS::Scissors, RPS::Rock),
        _ => panic!("unknown combo"),
    }
}
