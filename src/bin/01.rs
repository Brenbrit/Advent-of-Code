pub fn part_one(input: &str) -> Option<u32> {
    let mut split = input.split("\n");

    for s in split {
        if s == "" {
            println!("HI");
        }
    }

    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

use std::io::BufReader;
fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input).unwrap(), 24000 as u32);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
