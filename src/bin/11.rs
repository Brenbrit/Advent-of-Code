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
    items: RefCell<Vec<u64>>,
    operation: Expression,
    test: Test,
}

impl Monkey {
    fn pop_next_item(&mut self) -> Option<u64> {
        let item = self.items.borrow().get(0)?.clone();
        self.items.borrow_mut().remove(0);
        Some(item)
    }

    fn peek_next_item(&self) -> Option<u64> {
        Some(self.items.borrow().get(0)?.clone())
    }

    fn push_item(&mut self, item: u64) {
        self.items.borrow_mut().push(item)
    }

    fn compute_new(&self, old: u64) -> u64 {
        let first_term = match self.operation.left_hand_side {
            OperationTerm::Constant(c) => c,
            OperationTerm::Old => old,
        };
        let second_term = match self.operation.right_hand_side {
            OperationTerm::Constant(c) => c,
            OperationTerm::Old => old,
        };
        
        match self.operation.operation {
            Operation::Plus => first_term + second_term,
            Operation::Times => first_term * second_term,
        }
    }

    fn test_item(&self, item_worry_level: u64) -> bool {
        item_worry_level % self.test.divisible_by == 0
    }

    fn item_destination(&self, item_worry_level: u64) -> usize {
        if self.test_item(item_worry_level) {
            self.test.r#true
        } else {
            self.test.r#false
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct Expression {
    left_hand_side: OperationTerm,
    operation: Operation,
    right_hand_side: OperationTerm,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum OperationTerm {
    Constant(u64),
    Old,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum Operation {
    Plus,
    Times,
}

#[derive(PartialEq, PartialOrd, Debug, Clone)]
struct Test {
    divisible_by: u64,
    r#true: usize,
    r#false: usize,
}

pub fn part_one(input: &str) -> Option<u64> {
    // Read monkeys
    let monkeys = read_monkeys(input)?;
    // Perform throws and count inspections
    let mut inspection_counts = count_inspects(monkeys, 20);
    // Sort inspections
    inspection_counts.sort();

    Some(inspection_counts.pop()? * inspection_counts.pop()?)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn count_inspects(mut monkeys: Vec<Monkey>, rounds: usize) -> Vec<u64> {
    let mut inspection_counts: Vec<u64> = vec![0; monkeys.len()];

    // For each round,
    for _ in 0..rounds {
        // For each monkey (in order),
        for current_monkey_index in 0..monkeys.len() {
            // For each item in the monkey's hands,
            loop {
                match monkeys.get(current_monkey_index).unwrap().peek_next_item() {
                    Some(item_worry_level) => {
                        let mut item_worry_level = monkeys
                            .get(current_monkey_index)
                            .unwrap()
                            .compute_new(item_worry_level);
                        item_worry_level /= 3;
                        let item_destination = monkeys
                            .get(current_monkey_index)
                            .unwrap()
                            .item_destination(item_worry_level);
                        // Remove item from first monkey's posession
                        {
                            monkeys.get_mut(current_monkey_index).unwrap().pop_next_item();
                            inspection_counts[current_monkey_index] += 1;
                        }
                        {
                            monkeys.get_mut(item_destination).unwrap().push_item(item_worry_level);
                        }
                    },
                    None => { break },
                }
            }
        }
    }

    inspection_counts
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
        let starting_items: Vec<u64> = starting_items
            .last()?
            .trim()
            .split(", ")
            .into_iter()
            .map(|x| x.parse::<u64>().expect("Failed to interpret starting item"))
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
                    constant.parse::<u64>()
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
                    constant.parse::<u64>()
                        .expect("Failed to interpret constant")
                )
            },
        };

        let divisible_by: Vec<&str> = (*monkey_group.get(3)?)
            .split(" ")
            .collect();
        let divisible_by = divisible_by
            .last()?
            .parse::<u64>()
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
        assert_eq!(part_one(&input), Some(10605 as u64));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
