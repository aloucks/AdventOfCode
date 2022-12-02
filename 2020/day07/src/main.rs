use std::collections::HashMap;

use utility::{get_file_as_vec_string, get_input_path, Result};

fn main() -> Result<()> {
    let input_path = get_input_path()?;
    let contents = get_file_as_vec_string(&input_path)?;

    let map_1 = parse_input(&contents);

    let mut holder: Vec<String> = vec![];

    for (k, v) in map_1 {
        if v.contains(&"shiny gold".to_string()) {
            holder.push(k.clone());
        }
    }

    println!("{:?}", holder);

    Ok(())
}

fn parse_input(input: &Vec<String>) -> HashMap<String, Vec<String>> {
    let input_1 = input
        .iter()
        .map(|l| l.split("contain").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut map_1 = HashMap::new();

    input_1.iter().for_each(|data| {
        let outer_bag = get_bag_name(data[0]);

        let inner_bags = data[1]
            .trim_end_matches(".")
            .split(",")
            .map(|b| get_bag_name(b))
            .collect::<Vec<String>>();

        if inner_bags.len() == 1 && inner_bags[0] == "no other" {
            map_1.insert(outer_bag, vec![]);
        } else {
            println!("{:?}", inner_bags);
            let inner_bags_1 = inner_bags
                .iter()
                .map(|i| i.trim().splitn(2, ' ').nth(1).unwrap().to_string())
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
