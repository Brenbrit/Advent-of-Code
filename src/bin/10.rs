const SCREEN_RESOLUTION: [u32; 2] = [40, 6];

#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum Command {
    Addx(i32),
    NoOp,
}

pub fn part_one(input: &str) -> Option<i32> {
    let commands = read_commands(input)?;
    let during_cycle = calc_during_cycles(&commands);

    let mut cycle_num = 20;
    let mut score = 0;
    while cycle_num < during_cycle.len() {
        score += during_cycle.get(cycle_num-1).unwrap() * (cycle_num as i32);
        cycle_num += 40;
    }

    Some(score)
}

pub fn part_two(input: &str) -> Option<String> {
    let commands = read_commands(input)?;
    let during_cycle = calc_during_cycles(&commands);

    let horizontal_resolution = SCREEN_RESOLUTION[0];
    let vertical_resolution = SCREEN_RESOLUTION[1];
    let mut output = String::new();

    for line_num in 0..vertical_resolution {
        for col_num in 0..horizontal_resolution {
            let cycle_num = (horizontal_resolution * line_num) + col_num;
            let sprite_pos = during_cycle.get(cycle_num as usize)?;
            if ((col_num as i32) - sprite_pos).abs() <= 1 {
                output.push('#');
            } else {
                output.push('.');
            }
        }
        output.push('\n');
    }
    // remove last newline
    output.pop();

    Some(output)
}

fn calc_during_cycles(commands: &Vec<Command>) -> Vec<i32> {
    let mut during_cycle: Vec<i32> = vec![1];

    for command in commands.iter() {
        match command {
            Command::Addx(num) => {
                during_cycle.push(during_cycle.last().unwrap().clone());
                during_cycle.push(during_cycle.last().unwrap() + num);
            },
            Command::NoOp => {
                during_cycle.push(during_cycle.last().unwrap().clone());
            },
        }
    }

    during_cycle
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
        const PART_TWO_ANSWER: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(String::from(PART_TWO_ANSWER)));
    }
}
