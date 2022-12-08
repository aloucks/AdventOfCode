#![warn(dead_code)]
use std::{collections::HashMap, fs::read_to_string};

fn main() -> utility::Result<()> {
    let input_path = utility::get_input_path()?;
    let input_contents = read_to_string(&input_path)?;
    parse_3(&input_contents);

    Ok(())
}

fn parse_3(input: &str) {
    let mut tree: HashMap<String, Vec<DirOrFile>> = HashMap::new();
    let mut current_directory = String::new();

    let commands: Vec<Vec<&str>> = input
        .split('$')
        .skip(1)
        .map(|c| c.trim().split('\n').collect::<Vec<&str>>())
        .collect();

    commands.iter().for_each(|c| {
        let command = c[0].trim();

        if command.starts_with("cd") {
            if command == "cd .." {
                current_directory = previous_directory(&current_directory);
            } else {
                current_directory = new_directory(command, &current_directory);
            }
        }

        if command == "ls" {
            let files: Vec<_> = c[1..]
                .iter()
                .map(|e| {
                    let parts = e.split_once(' ').unwrap();

                    match parts.0 {
                        "dir" => DirOrFile::Dir(String::from(parts.1)),
                        _ => DirOrFile::File(String::from(parts.1), parts.0.parse().unwrap()),
                    }
                })
                .collect();

            tree.insert(current_directory.clone(), files);
        }
    });

    let mut sizes = HashMap::new();

    for k in tree.keys() {
        compute_size(&tree, &mut sizes, k);
    }

    let p1: u32 = sizes.iter().filter(|s| s.1 <= &100000).map(|s| s.1).sum();

    println!("p1: {}", p1);

    let used = sizes["/"];
    let p2 = sizes
        .iter()
        .filter(|s| used - s.1 <= 40000000)
        .map(|s| s.1)
        .min()
        .unwrap();

    println!("p2: {}", p2);
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
    result.push('/');
    result.push_str(dir);

    result
}

fn previous_directory(current_directory: &str) -> String {
    let parts = current_directory.trim().split('/').collect::<Vec<&str>>();
    let result = parts[0..parts.len() - 1].join("/");

    if result.is_empty() {
        return String::from("/");
    }

    result
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
                    new_dir.push('/');
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
