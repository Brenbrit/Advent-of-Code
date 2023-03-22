use std::collections::HashMap;

const COMMAND_START: &str = "$ ";

#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum Command {
    CD(String),
    LS(Vec<String>, Vec<File>),
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

pub fn part_one(input: &str) -> Option<usize> {
    let commands = read_commands(input)?;
    let root = interpret_commands(commands)?;

    dbg!(&root);

    None
}

pub fn part_two(_input: &str) -> Option<usize> {
    None
}

fn read_commands(input: &str) -> Option<Vec<Command>> {
    // Split by '$ '
    let split: Vec<&str> = input.split(COMMAND_START).collect();
    let mut commands: Vec<Command> = Vec::new();

    for command_str in split.iter() {

        if command_str.starts_with("cd") {
            let cd_split: Vec<&str> = command_str.split(" ").collect();
            let cd_destination = cd_split.get(1)?;
            commands.push(Command::CD(String::from((*cd_destination).trim())));
            continue;

        } else if command_str.starts_with("ls") {
            let mut dirs: Vec<String> = Vec::new();
            let mut files: Vec<File> = Vec::new();

            let lines: Vec<&str> = command_str.lines().collect();
            for line in lines.iter().skip(1) {
                let line_split: Vec<&str> = line.split(" ").collect();
                let first_word = *line_split.get(0)?;
                let second_word = *line_split.get(1)?;
                if first_word == "dir" {
                    dirs.push(String::from(second_word));
                } else {
                    let file_size = first_word.parse::<usize>()
                    .expect("Failed to interpret size");
                    files.push(File { name: String::from(second_word), size: file_size });
                }
            }

            commands.push(Command::LS(dirs, files));
        }
    }

    Some(commands)
}

fn interpret_commands(commands: Vec<Command>) -> Option<HashMap<String, Option<usize>>> {
    let mut file_system: HashMap<String, Option<usize>> = HashMap::new();
    let mut current_directory = "/".to_owned();

    for command in commands {
        match command {
            Command::CD(new_directory) => {

                if new_directory.as_str() == ".." {
                    let current_path_split: Vec<&str> = current_directory.split("/").collect();
                    let (_, before_last) = current_path_split.split_last().unwrap();
                    current_directory = before_last.join("/");
                } else if new_directory.as_str() == "/" {
                    current_directory = "".to_owned();
                } else {
                    current_directory = [current_directory.as_str(), new_directory.as_str()].join("/");
                }
            },

            Command::LS(_, files) => {
                // We don't care about subdirs.
                // If we later visit them and call ls, THEN we'll care.
                file_system.insert(current_directory.clone(), None);
                for file in files {
                    let file_path = [current_directory.as_str(), file.name.as_str()].join("/");
                    file_system.insert(file_path, Some(file.size));
                }
            },
        }
    }

    Some(file_system)
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
        assert_eq!(part_one(&input), Some(95437 as usize));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
