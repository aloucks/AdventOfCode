#![warn(dead_code)]
use std::{collections::HashMap, fs::read_to_string};

fn main() -> utility::Result<()> {
    let input_path = utility::get_input_path()?;
    let input_data = utility::get_file_as_vec_string(&input_path)?;

    parse_2(&input_data);

    let input_contents = read_to_string(&input_path)?;
    //parse_3(&input_contents);

    Ok(())
}

fn parse_3(input: &str) {
    let mut current_directory = String::new();

    let commands: Vec<Vec<&str>> = input
        .split('$')
        .skip(1)
        .map(|c| c.trim().split('\n').collect::<Vec<&str>>())
        .collect();

    commands.iter().for_each(|c| {
        let command = c[0].trim();

        if command.starts_with("cd") {
            println!("previous_directory: {}", &current_directory);

            if command == "cd .." {
                current_directory = previous_directory(&current_directory);
            } else {
                current_directory = new_directory(command, &current_directory);
            }

            println!("current_directory: {}", &current_directory);
        }
    });

    // println!("{:#?}", commands2);
}

fn new_directory(input: &str, current_directory: &str) -> String {
    let dir = input.split(' ').nth(1).unwrap();

    if current_directory.is_empty() && dir == "/" {
        return String::from("/");
    }

    let mut current_directory = current_directory;

    if current_directory == "/" {
        current_directory = "";
    }

    let mut result = String::from(current_directory);
    result.push_str("/");
    result.push_str(dir);

    result
}

fn previous_directory(current_directory: &str) -> String {
    let parts = current_directory.trim().split("/").collect::<Vec<&str>>();
    let result = parts[0..parts.len() - 1].join("/");

    if result.is_empty() {
        return String::from("/");
    }

    result
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

                    cur_dir = previous_directory(&cur_dir);
                } else {
                    tree.entry(cur_dir.clone())
                        .or_insert(Vec::new())
                        .extend(cur_files.clone());
                    cur_files.clear();

                    cur_dir = new_directory(&l.trim().replace("$ ", ""), &cur_dir);
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

#[cfg(test)]
mod tests {
    use super::new_directory;
    use super::previous_directory;

    #[test]
    fn current_directory_empty_and_dir_slash() {
        let result = new_directory("cd /", "");

        assert_eq!(result, "/");
    }

    #[test]
    fn current_directory_empty_and_dir_not_slash() {
        let result = new_directory("cd b", "");

        assert_eq!(result, "/b");
    }

    #[test]
    fn current_directory_slash_and_dir_not_empty() {
        let result = new_directory("cd b", "/");

        assert_eq!(result, "/b");
    }

    #[test]
    fn current_directory_slash_something_and_dir_not_empty() {
        let result = new_directory("cd b", "/a");

        assert_eq!(result, "/a/b");
    }

    #[test]
    fn previous_directory_current_dir_empty() {
        let result = previous_directory("");

        assert_eq!(result, "/");
    }

    #[test]
    fn previous_directory_current_dir_slash() {
        let result = previous_directory("/");

        assert_eq!(result, "/");
    }

    #[test]
    fn previous_directory_current_dir_slash_something() {
        let result = previous_directory("/b");

        assert_eq!(result, "/");
    }

    #[test]
    fn previous_directory_slash_something_slash_sometthing() {
        let result = previous_directory("/a/b");

        assert_eq!(result, "/a");
    }

    #[test]
    fn previous_directory_slash_something_slash_sometthing_slash_something() {
        let result = previous_directory("/a/b/c");

        assert_eq!(result, "/a/b");
    }
}
