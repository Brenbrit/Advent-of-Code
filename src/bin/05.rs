const EMPTY_CRATE_LABEL: char = ' ';
// instruction constants
const INSTRUCTION_NUM_CRATES_LOCATION: usize = 1;
const INSTRUCTION_FROM_LOCATION: usize = 3;
const INSTRUCTION_TO_LOCATION: usize = 5;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct Instruction {
    from: usize, // as an index, 0-based
    to: usize, // as an index, 0-based
    num_crates: u32,
}

enum CraneEdition {
    CrateMover9000,
    CrateMover9001,
}

pub fn part_one(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    let crates = read_crates(&lines).unwrap();
    let instructions = read_instructions(&lines).unwrap();

    let end_state = execute_instructions(&crates, instructions, &CraneEdition::CrateMover9000)?;
    let mut top_crates = String::new();
    for stack in end_state {
        match stack.last() {
            Some(crate_id) => {
                top_crates.push(*crate_id);
            },
            None => (),
        }
    }

    Some(top_crates)
}

pub fn part_two(input: &str) -> Option<String> {
    let lines: Vec<&str> = input.lines().collect();
    let crates = read_crates(&lines).unwrap();
    let instructions = read_instructions(&lines).unwrap();

    let end_state = execute_instructions(&crates, instructions, &CraneEdition::CrateMover9001)?;
    let mut top_crates = String::new();
    for stack in end_state {
        match stack.last() {
            Some(crate_id) => {
                top_crates.push(*crate_id);
            },
            None => (),
        }
    }

    Some(top_crates)
}

fn get_starting_max_height(lines: &Vec<&str>) -> Option<u32> {
    let mut height: usize = 0;
    while height < lines.len() {
        if (String::from(*lines.get(height)?)).contains('[') {
            height += 1;
            continue;
        } else {
            return Some(height as u32);
        }
    }

    None
}

fn read_crates(lines: &Vec<&str>) -> Option<Vec<Vec<char>>> {
    let num_stacks = ((String::from(*lines.first()?).len() + 1) / 4) as u32;
    let max_stack_height = get_starting_max_height(lines)?;

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for stack_num in 0..num_stacks {
        let mut this_stack: Vec<char> = Vec::new();

        for row in (0..(max_stack_height as usize)).rev() {
            let crate_label_position = ((4 * stack_num) + 1) as usize;
            let crate_label = lines.get(row)?.chars().nth(crate_label_position)?;

            if crate_label == EMPTY_CRATE_LABEL {
                break;
            }
            
            this_stack.push(crate_label);
        }

        stacks.push(this_stack);
    }

    Some(stacks)
}

fn read_instructions(lines: &Vec<&str>) -> Option<Vec<Instruction>> {
    // instructions start on line max_stack_height + 3
    let max_stack_height = get_starting_max_height(lines)?;

    let mut instructions: Vec<Instruction> = Vec::new();

    for line_num in ((max_stack_height + 2) as usize)..lines.len() {
        let line_split: Vec<&str> = lines.get(line_num)?.split(' ').collect();

        let num_crates = (*line_split.get(INSTRUCTION_NUM_CRATES_LOCATION)?).parse::<u32>().unwrap();
        let from = (*line_split.get(INSTRUCTION_FROM_LOCATION)?).parse::<usize>().unwrap() - 1;
        let to = (*line_split.get(INSTRUCTION_TO_LOCATION)?).parse::<usize>().unwrap() - 1;

        let instruction = Instruction{
            num_crates,
            from,
            to,
        };

        instructions.push(instruction);
    }

    Some(instructions)
}

fn execute_instructions(input_crates: &Vec<Vec<char>>, instructions: Vec<Instruction>, crane_edition: &CraneEdition) -> Option<Vec<Vec<char>>> {

    // Convert Vec<Vec<char>> to Vec<String> to ignore mutability rules
    // Very frustrating. I am not sure how to avoid this.
    let mut crates: Vec<String> = Vec::new();
    for stack in input_crates.iter() {
        crates.push(
            stack.iter().cloned().collect::<String>()
        );
    }

    // Move crates!
    for instruction in instructions {
        let mut moved_crates: Vec<char> = Vec::new();
        for _ in 0..(instruction.num_crates) {

            // We get the value of the top crate inside a scope
            // so that we can borrow crates as mutable again.
            let crate_id: char;
            {
                crate_id = crates.get_mut(instruction.from)?.pop()?;
            }
            moved_crates.push(crate_id);
            //crates.get_mut(instruction.to)?.push(crate_id);
        }

        crates.get_mut(instruction.to)?.push_str(
            match crane_edition {
                CraneEdition::CrateMover9000 => moved_crates,
                CraneEdition::CrateMover9001 => {
                    moved_crates.reverse();
                    moved_crates
                }
            }.iter().cloned().collect::<String>().as_str()
        );
    }

    let mut crates_as_chars: Vec<Vec<char>> = Vec::new();
    for stack in crates {
        crates_as_chars.push(stack.chars().collect());
    }

    Some(crates_as_chars)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
