use std::{collections::VecDeque, fs::read_to_string};

fn main() -> utility::Result<()> {
    let input_path = utility::get_input_path()?;
    let input_contents = read_to_string(&input_path)?;

    parse_1(&input_contents);
    parse_2(&input_contents);

    Ok(())
}

fn parse_1(input: &str) {
    let delims = utility::get_file_delimiters(input);
    let parts = input.split_once(delims.1).unwrap();

    // let mut cargo = parse_containers(parts.0);
    let instructions = parse_instructions(parts.1);
    let mut stacks = parse_stacks(parts.0);

    println!("{:#?}", stacks);

    for instruction in &instructions {
        for _ in 0..instruction[0] {
            // 0 = how many moves
            let from = instruction[1] - 1; // from stack
            let to = instruction[2] - 1; // to stack

            let to_move: String;

            {
                let from_stack = &mut stacks[from];
                to_move = from_stack.pop_front().unwrap();
            }

            let to_stack = &mut stacks[to];
            to_stack.push_front(to_move);
        }
    }

    println!("{:#?}", stacks);

    let tops = stacks
        .iter()
        .map(|x| {
            x.front()
                .unwrap()
                .replace("[", "")
                .replace("]", "")
                .trim()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("");

    println!("{:?}", tops);
}

fn parse_2(input: &str) {
    let delims = utility::get_file_delimiters(input);
    let parts = input.split_once(delims.1).unwrap();

    // let mut cargo = parse_containers(parts.0);
    let instructions = parse_instructions(parts.1);
    let mut stacks = parse_stacks(parts.0);

    println!("{:#?}", stacks);

    for instruction in &instructions {
        let mut to_move: VecDeque<String> = VecDeque::from(vec![]);

        for _ in 0..instruction[0] {
            let from_stack = &mut stacks[instruction[1] - 1];
            let moved = from_stack.pop_front().unwrap();
            to_move.push_back(moved);
        }

        let to_stack = &mut stacks[instruction[2] - 1];
        for _ in 0..to_move.len() {
            let moved = to_move.pop_back().unwrap();
            to_stack.push_front(moved);
        }

        println!("{:#?}", stacks);
    }

    println!("{:#?}", stacks);

    let tops = stacks
        .iter()
        .map(|x| {
            x.front()
                .unwrap()
                .replace("[", "")
                .replace("]", "")
                .trim()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("");

    println!("{:?}", tops);
}

fn parse_stacks(input: &str) -> Vec<VecDeque<String>> {
    let stack_count = input
        .lines()
        .last()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .count();

    let mut stacks: Vec<VecDeque<String>> = vec![VecDeque::from(vec![]); stack_count];

    input.lines().filter(|l| l.contains("[")).for_each(|i| {
        let chars = i.chars().collect::<Vec<_>>();
        let chars = chars.chunks(4).collect::<Vec<_>>();
        let chars = chars
            .iter()
            .map(|l| String::from_iter(l.iter()))
            .collect::<VecDeque<_>>();

        for (idx, cr) in chars.iter().enumerate() {
            if !cr.trim().is_empty() {
                let stack = &mut stacks[idx];
                stack.push_back(cr.to_string());
            }
        }
    });

    stacks
}

fn parse_instructions(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.replace("move ", "")
                .replace(" from", "")
                .replace(" to", "")
                .split_ascii_whitespace()
                .map(|d| d.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
