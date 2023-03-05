use std::{collections::BTreeMap, vec};

#[derive(Debug)]
enum Command<'a> {
    Cd(Cd<'a>),
    Ls(Vec<DataType<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug, Clone)]
enum DataType<'a> {
    File(u32),
    Dir(&'a str),
}

fn ls(input: &str) -> Command {
    let command = input
        .trim()
        .lines()
        .skip(1)
        .filter_map(|x| {
            if let Some((size_or_type, name)) = x.split_once(' ') {
                match size_or_type {
                    "dir" => Some(DataType::Dir(name)),
                    file_size if file_size.chars().all(|x| x.is_alphanumeric()) => {
                        Some(DataType::File(file_size.parse::<u32>().unwrap()))
                    }
                    _ => None,
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Command::Ls(command)
}

fn cd(input: &str) -> Command {
    let (_, dir) = input.trim().split_once("cd ").unwrap();
    match dir {
        "/" => Command::Cd(Cd::Root),
        ".." => Command::Cd(Cd::Up),
        name => Command::Cd(Cd::Down(name)),
    }
}

fn parse_commands(input: &str) -> Vec<Command> {
    input
        .trim()
        .split("$ ")
        .skip(1)
        .map(|line| {
            if line.starts_with("ls") {
                ls(line)
            } else {
                cd(line)
            }
        })
        .collect::<Vec<_>>()
}

fn build_directories(commands: Vec<Command>) -> BTreeMap<String, u32> {
    let mut cwd = vec![];
    commands
        .into_iter()
        .map(|command| {
            let mut total_size = 0;
            match command {
                Command::Cd(Cd::Root) => {
                    cwd.clear();
                    cwd.push("");
                }
                Command::Cd(Cd::Down(dir)) => {
                    cwd.push(dir);
                }
                Command::Cd(Cd::Up) => {
                    if !cwd.is_empty() {
                        cwd.pop();
                    }
                }
                Command::Ls(items) => {
                    items.into_iter().for_each(|item| {
                        if let DataType::File(size) = item {
                            total_size += size
                        }
                    });
                }
            }
            (cwd.clone().join("/"), total_size)
        })
        .filter(|(_, size)| *size > 0)
        .collect()
}

fn calculate_sizes(mut directories: BTreeMap<String, u32>) -> BTreeMap<String, u32> {
    for (path, size) in directories.clone() {
        let dirs = path.split('/').collect::<Vec<&str>>();

        dirs.iter().rev().skip(1).enumerate().for_each(|(i, _)| {
            let cwd = dirs[0..=i].join("/");
            directories
                .entry(cwd)
                .and_modify(|v| *v += size)
                // some folders don't have size set beforehand, because they contain only other directories,
                // so to adress that issue we set it size to current size
                .or_insert(size);
        });
    }
    directories
}

pub fn part_one(input: &str) -> Option<u32> {
    let commands = parse_commands(input);
    let directories = calculate_sizes(build_directories(commands));

    let size = directories
        .values()
        .filter(|&size| *size <= 100_000)
        .sum::<u32>();

    Some(size)
}

pub fn part_two(input: &str) -> Option<u32> {
    let commands = parse_commands(input);
    let directories = calculate_sizes(build_directories(commands));

    const TOTAL_SPACE: u32 = 70_000_000;
    const REQUIRED_SPACE: u32 = 30_000_000;

    let available_space = TOTAL_SPACE - directories.values().max().unwrap();
    let needed_space = REQUIRED_SPACE - available_space;

    directories
        .values()
        .filter(|&size| size >= &needed_space)
        .min()
        .copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
