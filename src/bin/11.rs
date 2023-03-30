use std::cell::RefCell;

/*
Example:
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
 */
#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct Monkey {
    items: RefCell<Vec<u32>>,
    operation: Expression,
    test: Test,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct Expression {
    left_hand_side: OperationTerm,
    operation: Operation,
    right_hand_side: OperationTerm,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum OperationTerm {
    Constant(i32),
    Old,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum Operation {
    Plus,
    Times,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct Test {
    divisible_by: u32,
    r#true: usize,
    r#false: usize,
}

pub fn part_one(input: &str) -> Option<u32> {
    let monkeys = read_monkeys(input)?;
    dbg!(monkeys);

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn read_monkeys(input: &str) -> Option<Vec<Monkey>> {

    let split: Vec<&str> = input.lines().collect();
    let monkey_groups = split.chunks(7);

    let mut monkeys: Vec<Monkey> = vec![];

    for monkey_group in monkey_groups {

        let starting_items: Vec<&str> = monkey_group
            .get(1)?
            .split("Starting items:")
            .collect();
        let starting_items: Vec<u32> = starting_items
            .last()?
            .trim()
            .split(", ")
            .into_iter()
            .map(|x| x.parse::<u32>().expect("Failed to interpret starting item"))
            .collect();

        let operation_expr = monkey_group
            .get(2)?
            .split("Operation: new = ")
            .last()?;
        let operation_parts: Vec<&str> = operation_expr.split(" ").collect();
        let monkey_operation = Expression{
            left_hand_side: match *operation_parts.get(0)? {
                "old" => OperationTerm::Old,
                constant => OperationTerm::Constant(
                    constant.parse::<i32>()
                        .expect("Failed to interpret constant")
                )
            },
            operation: match *operation_parts.get(1)? {
                "+" => Operation::Plus,
                "*" => Operation::Times,
                _ => {
                    println!("Failed to interpret operation");
                    return None;
                }
            },
            right_hand_side: match *operation_parts.get(2)? {
                "old" => OperationTerm::Old,
                constant => OperationTerm::Constant(
                    constant.parse::<i32>()
                        .expect("Failed to interpret constant")
                )
            },
        };

        let divisible_by: Vec<&str> = (*monkey_group.get(3)?)
            .split(" ")
            .collect();
        let divisible_by = divisible_by
            .last()?
            .parse::<u32>()
            .expect("Failed to interpret test -> divisible by");

        let true_throw: Vec<&str> = (*monkey_group.get(4)?)
            .split(" ")
            .collect();
        let true_throw = true_throw
            .last()?
            .parse::<usize>()
            .expect("Failed to interpret test -> true throw");

        let false_throw: Vec<&str> = (*monkey_group.get(5)?)
            .split(" ")
            .collect();
        let false_throw = false_throw
            .last()?
            .parse::<usize>()
            .expect("Failed to interpret test -> false throw");

        let monkey_test = Test{
            divisible_by: divisible_by,
            r#true: true_throw,
            r#false: false_throw,
        };

        let monkey = Monkey{
            items: RefCell::new(starting_items),
            operation: monkey_operation,
            test: monkey_test,
        };
        
        monkeys.push(monkey);
    }

    Some(monkeys)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
