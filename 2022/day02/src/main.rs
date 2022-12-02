use std::str::FromStr;

fn main() -> utility::Result<()> {
    let input_file = utility::get_input_path()?;
    let input = utility::get_file_as_vec_string(&input_file)?;

    let input_1 = input
        .iter()
        .map(|l| line_to_rps_vec(l))
        .collect::<Vec<Vec<RPS>>>();

    let score: u32 = input_1.iter().map(|l| rps_vec_to_score(l)).sum();

    println!("{:?}", score);

    let input_1 = input
        .iter()
        .map(|l| line_to_rps2_vec(l))
        .collect::<Vec<Vec<RPS2>>>();

    let score: u32 = input_1.iter().map(|l| rps2_vec_to_score(l)).sum();

    println!("{:?}", score);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for RPS {
    type Err = utility::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPS::Rock),
            "B" => Ok(RPS::Paper),
            "C" => Ok(RPS::Scissors),
            "X" => Ok(RPS::Rock),
            "Y" => Ok(RPS::Paper),
            "Z" => Ok(RPS::Scissors),
            _ => Err(utility::format_err!("unknown value: {}", s)),
        }
    }
}

fn line_to_rps_vec(input: &str) -> Vec<RPS> {
    let plays = input
        .split_ascii_whitespace()
        .map(|p| p.parse().unwrap())
        .collect::<Vec<RPS>>();

    plays
}

fn rps_vec_to_score(input: &Vec<RPS>) -> u32 {
    if input[0] == input[1] {
        return rps_to_value(&input[1]) + 3;
    }

    if input[0] == RPS::Rock && input[1] == RPS::Paper {
        return 6 + rps_to_value(&input[1]);
    }

    if input[0] == RPS::Rock && input[1] == RPS::Scissors {
        return 0 + rps_to_value(&input[1]);
    }

    if input[0] == RPS::Paper && input[1] == RPS::Rock {
        return 0 + rps_to_value(&input[1]);
    }

    if input[0] == RPS::Paper && input[1] == RPS::Scissors {
        return 6 + rps_to_value(&input[1]);
    }

    if input[0] == RPS::Scissors && input[1] == RPS::Rock {
        return 6 + rps_to_value(&input[1]);
    }

    if input[0] == RPS::Scissors && input[1] == RPS::Paper {
        return 0 + rps_to_value(&input[1]);
    }

    0
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

impl FromStr for RPS2 {
    type Err = utility::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RPS2::Rock),
            "B" => Ok(RPS2::Paper),
            "C" => Ok(RPS2::Scissors),
            "X" => Ok(RPS2::Lose),
            "Y" => Ok(RPS2::Draw),
            "Z" => Ok(RPS2::Win),
            _ => Err(utility::format_err!("unknown value: {}", s)),
        }
    }
}

fn line_to_rps2_vec(input: &str) -> Vec<RPS2> {
    let plays = input
        .split_ascii_whitespace()
        .map(|p| p.parse().unwrap())
        .collect::<Vec<RPS2>>();

    plays
}

fn rps2_vec_to_score(input: &Vec<RPS2>) -> u32 {
    if input[1] == RPS2::Win {
        if input[0] == RPS2::Rock {
            return 6 + rps2_to_value(&RPS2::Paper);
        }

        if input[0] == RPS2::Paper {
            return 6 + rps2_to_value(&RPS2::Scissors);
        }

        if input[0] == RPS2::Scissors {
            return 6 + rps2_to_value(&RPS2::Rock);
        }
    }

    if input[1] == RPS2::Lose {
        if input[0] == RPS2::Rock {
            return 0 + rps2_to_value(&RPS2::Scissors);
        }

        if input[0] == RPS2::Paper {
            return 0 + rps2_to_value(&RPS2::Rock);
        }

        if input[0] == RPS2::Scissors {
            return 0 + rps2_to_value(&RPS2::Paper);
        }
    }

    if input[1] == RPS2::Draw {
        if input[0] == RPS2::Rock {
            return 3 + rps2_to_value(&RPS2::Rock);
        }

        if input[0] == RPS2::Paper {
            return 3 + rps2_to_value(&RPS2::Paper);
        }

        if input[0] == RPS2::Scissors {
            return 3 + rps2_to_value(&RPS2::Scissors);
        }
    }

    0
}

fn rps2_to_value(input: &RPS2) -> u32 {
    match input {
        RPS2::Rock => 1,
        RPS2::Paper => 2,
        RPS2::Scissors => 3,
        _ => 0,
    }
}
