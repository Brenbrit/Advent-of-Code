pub fn part_one(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.lines().collect();

    let misplaced_items: Vec<char> = split.into_iter().map(
        |line| find_misplaced_item(line).unwrap()
    ).collect();

    let item_priorities: Vec<u32> = misplaced_items.into_iter().map(
        |item| item_priority(item)
    ).collect();

    Some(item_priorities.iter().sum())
}

fn find_misplaced_item(sack_contents: &str) -> Option<char> {

    let first_pouch = &sack_contents[..sack_contents.len()/2];
    let second_pouch: Vec<char> = sack_contents[sack_contents.len()/2..].chars().collect();
    
    for item in first_pouch.chars() {
        if second_pouch.contains(&item) {
            return Some(item);
        }
    }

    None
}

fn item_priority(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        (item as u32) - 38
    } else {
        (item as u32) - 96
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.lines().collect();

    let misplaced_items: Vec<char> = split.chunks(3).map(
        |lines| find_badge(lines).unwrap()
    ).collect();

    let item_priorities: Vec<u32> = misplaced_items.into_iter().map(
        |item| item_priority(item)
    ).collect();

    Some(item_priorities.iter().sum())
}

fn find_badge(lines: &[&str]) -> Option<char> {

    let first_pouch = lines[0];
    let second_pouch: Vec<char> = lines[1].chars().collect();
    let third_pouch: Vec<char> = lines[2].chars().collect();
    
    for item in first_pouch.chars() {
        if second_pouch.contains(&item) && third_pouch.contains(&item) {
            return Some(item);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70 as u32));
    }
}
