#![warn(dead_code)]
use std::{collections::HashMap, fs::read_to_string};

fn main() -> utility::Result<()> {
    let input_path = utility::get_input_path()?;
    //let input_data = utility::get_file_as_vec_string(&input_path)?;
    // parse_2(&input_data);

    let input_contents = read_to_string(&input_path)?;

    parse_3(&input_contents);

    Ok(())
}

fn parse_3(input: &str) {
    let commands: Vec<Vec<&str>> = input
        .split('$')
        .skip(1)
        .map(|c| c.trim().split('\n').collect::<Vec<&str>>())
        .collect();
    println!("{:#?}", commands);
}

fn parse_1(input: &Vec<String>) {
    let mut tree: HashMap<String, Vec<(u32, String)>> = HashMap::new();

    let mut cur_dir = String::from("/");
    let mut cur_files: Vec<(u32, String)> = vec![];

    let mut cur_command = String::from("$ ls");
    let mut prev_command = String::from("");

    input[2..input.len()].iter().for_each(|l| {
        if l.starts_with("$") {
            prev_command = cur_command.clone();
            cur_command = l.clone();

            if l.starts_with("$ ls") {}

            if l.starts_with("$ cd") {
                println!("cur_command: {}", cur_command);
                println!("cur_dir: {}", cur_dir);
                println!("files: {:?}", cur_files);

                if l.contains("..") {
                    if prev_command == "$ ls" {
                        tree.insert(cur_dir.clone(), cur_files.clone());
                        cur_files.clear();
                    }

                    let parts = cur_dir.split("/").collect::<Vec<&str>>();
                    cur_dir = parts[0..parts.len() - 1].join("/");

                    if cur_dir == "" {
                        cur_dir = String::from("/");
                    }
                } else {
                    tree.insert(cur_dir.clone(), cur_files.clone());
                    cur_files.clear();

                    let new_dir = l.split(" ").nth(2).unwrap();
                    if cur_dir.ends_with("/") {
                        cur_dir.push_str(new_dir);
                    } else {
                        cur_dir.push_str("/");
                        cur_dir.push_str(new_dir);
                    }
                }
            }
        } else {
            if !l.starts_with("dir") {
                let parts = l.split_once(" ").unwrap();
                let cur_file: (u32, String) = (parts.0.parse().unwrap(), String::from(parts.1));
                cur_files.push(cur_file);
            }
        }
    });

    tree.insert(cur_dir.clone(), cur_files.clone());
    cur_files.clear();

    println!("{:#?}", tree);

    for k in tree.keys() {}
}

fn parse_2(input: &Vec<String>) {
    let mut tree: HashMap<String, Vec<DirOrFile>> = HashMap::new();

    let mut cur_dir = String::from("/");
    let mut cur_files: Vec<DirOrFile> = vec![];

    let mut cur_command = String::from("$ ls");
    let mut prev_command = String::from("");

    input[2..input.len()].iter().for_each(|l| {
        if l.starts_with("$") {
            prev_command = cur_command.clone();
            cur_command = l.clone();

            if l.starts_with("$ ls") {}

            if l.starts_with("$ cd") {
                println!("cur_command: {}", cur_command);
                println!("cur_dir: {}", cur_dir);
                println!("files: {:?}", cur_files);

                if l.contains("..") {
                    if prev_command == "$ ls" {
                        tree.entry(cur_dir.clone())
                            .or_insert(Vec::new())
                            .extend(cur_files.clone());
                        cur_files.clear();
                    }

                    let parts = cur_dir.split("/").collect::<Vec<&str>>();
                    cur_dir = parts[0..parts.len() - 1].join("/");

                    if cur_dir == "" {
                        cur_dir = String::from("/");
                    }
                } else {
                    tree.entry(cur_dir.clone())
                        .or_insert(Vec::new())
                        .extend(cur_files.clone());
                    cur_files.clear();

                    let new_dir = l.split(" ").nth(2).unwrap();
                    if cur_dir.ends_with("/") {
                        cur_dir.push_str(new_dir);
                    } else {
                        cur_dir.push_str("/");
                        cur_dir.push_str(new_dir);
                    }
                }
            }
        } else {
            let parts = l.split_once(" ").unwrap();

            match parts.0 {
                "dir" => cur_files.push(DirOrFile::Dir {
                    0: String::from(parts.1),
                }),
                _ => cur_files.push(DirOrFile::File {
                    0: String::from(parts.1),
                    1: parts.0.parse().unwrap(),
                }),
            }
        }
    });

    tree.entry(cur_dir.clone())
        .or_insert(Vec::new())
        .extend(cur_files.clone());

    let mut sizes = HashMap::new();

    for k in tree.keys() {
        compute_size(&tree, &mut sizes, k);
    }

    let s: u32 = sizes.iter().filter(|s| s.1 <= &100000).map(|s| s.1).sum();

    let used = sizes["/"];
    let t = sizes
        .iter()
        .filter(|s| used - s.1 <= 40000000)
        .map(|s| s.1)
        .min()
        .unwrap();

    println!("{}", used);
    println!("{}", t);
}

fn compute_size(
    tree: &HashMap<String, Vec<DirOrFile>>,
    sizes: &mut HashMap<String, u32>,
    path: &str,
) {
    let computed_size: u32 = tree
        .get(path)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|i| match i {
            DirOrFile::Dir(dir) => {
                let mut new_dir = String::from(path);
                if path != "/" {
                    new_dir.push_str("/");
                }
                new_dir.push_str(dir);
                compute_size(tree, sizes, &new_dir);
                *sizes.get(&new_dir).unwrap()
            }
            DirOrFile::File(_, size) => *size,
        })
        .sum();

    sizes.insert(path.to_string(), computed_size);
}

#[derive(Debug, Clone)]
enum DirOrFile {
    Dir(String),
    File(String, u32),
}
