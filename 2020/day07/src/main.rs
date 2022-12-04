use std::collections::HashMap;

use utility::{get_file_as_vec_string, get_input_path, Result};

fn main() -> Result<()> {
    let input_path = get_input_path()?;
    let contents = get_file_as_vec_string(&input_path)?;

    let map_1 = parse_input(&contents);

    let mut look = vec!["shiny gold".to_string()];
    let mut count = vec![];

    while !look.is_empty() {
        let mut next_look = vec![];
        for (k, v) in &map_1 {
            if v.iter().any(|i| look.contains(i)) {
                next_look.push(k.clone());
            }
        }

        look.clear();
        next_look.iter().for_each(|f| look.push(f.clone()));

        next_look.iter().for_each(|f| count.push(f.clone()));
    }

    count.sort_unstable();
    count.dedup();
    println!("{:?}", count.len());

    let parsed_contents = contents.iter().map(|l| parse_line(l)).collect::<Vec<_>>();
    let the_sum = contained(&parsed_contents, "shiny gold") - 1;
    println!("{}", the_sum);

    Ok(())
}

fn contained(bag_data: &Vec<(String, Vec<(usize, String)>)>, color: &str) -> usize {
    let bag = bag_data.iter().find(|bag| bag.0 == color).unwrap();
    let child_sum: usize = bag
        .1
        .iter()
        .map(|child_bag| {
            let child_amount = child_bag.0;
            let child_color = child_bag.1.clone();
            if child_amount == 0 {
                return 1;
            }

            let inside = contained(bag_data, &child_color);
            match inside {
                0 => child_amount,
                _ => child_amount * inside,
            }
        })
        .sum();

    child_sum + 1
}

fn parse_line(input: &str) -> (String, Vec<(usize, String)>) {
    let mut split_line = input.split(" bags contain ");
    let bag_color = split_line.next().unwrap().to_string();

    let contents = split_line
        .next()
        .map(|l| match l {
            "no other bags." => vec![],
            _ => l.split(", ").map(parse_contents).collect(),
        })
        .unwrap();

    (bag_color, contents)
}

fn parse_contents(input: &str) -> (usize, String) {
    let number: usize = input[..1].parse().unwrap();
    let result = input[2..]
        .replace(" bags.", "")
        .replace(" bag.", "")
        .replace(" bags", "")
        .replace(" bag", "");

    (number, result)
}

fn parse_input(input: &[String]) -> HashMap<String, Vec<String>> {
    let input_1 = input
        .iter()
        .map(|l| l.split("contain").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut map_1 = HashMap::new();

    input_1.iter().for_each(|data| {
        let outer_bag = get_bag_name(data[0]);

        let inner_bags = data[1]
            .trim_end_matches('.')
            .split(',')
            .map(get_bag_name)
            .collect::<Vec<String>>();

        if inner_bags.len() == 1 && inner_bags[0] == "no other" {
            map_1.insert(outer_bag, vec![]);
        } else {
            let inner_bags_1 = inner_bags
                .iter()
                .map(|i| i.trim().split_once(' ').unwrap().1.to_string())
                .collect::<Vec<String>>();
            map_1.insert(outer_bag, inner_bags_1);
        }
    });

    map_1
}

fn get_bag_name(input: &str) -> String {
    input
        .replace("bags", "")
        .replace("bag", "")
        .trim()
        .to_string()
}
