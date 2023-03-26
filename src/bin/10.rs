#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum Command {
    Addx(i32),
    NoOp,
}

pub fn part_one(input: &str) -> Option<i32> {
    let commands = read_commands(input)?;

    let mut register_values: Vec<i32> = vec![];
    let mut register: i32 = 1;

    let mut command_num = 1;
    for command in commands {

        if (command_num - 20) % 40 == 0 {
            register_values.push(register * command_num);
            println!("After cycle {:?}, register has value {:?}", command_num, register * command_num);
        }

        match command {
            Command::Addx(val) => { register += val; }
            _ => {}
        }
        command_num += 1;

        command_num += 1;
    }

    Some(register_values.iter().sum())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn read_commands(input: &str) -> Option<Vec<Command>> {
    let lines = input.lines();
    let mut to_return: Vec<Command> = vec![];

    for line in lines {
        let line_split: Vec<&str> = line.split(' ').collect();
        let command_type = *line_split.get(0)?;
        match command_type {
            "noop" => {
                to_return.push(Command::NoOp);
            },
            "addx" => {
                // Push a noop and then an addx
                to_return.push(Command::NoOp);
                to_return.push(Command::Addx(line_split.get(1)?.parse().expect("Failed to interpret addx arg as number")))
            },
            _ => {
                println!("Failed to interpret line!!!!!");
            },
        }
    }

    Some(to_return)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
