pub fn part_one(input: &str) -> Option<u32> {
    let split = input.lines();

    let mut max = 0;
    let mut current_total = 0;

    for s in split {
        match s.parse::<u32>() {
            Ok(num) => current_total += num,
            Err(_) => {
                if current_total > max {
                    max = current_total;
                }
                current_total = 0
            },
        }
    }

    Some(max)
}

pub fn part_two(input: &str) -> Option<u32> {
    let split = input.lines();

    let mut maxes = Vec::new();
    let mut current_total = 0;

    for s in split {
        match s.parse::<u32>() {
            Ok(num) => current_total += num,
            Err(_) => {
                maxes.push(current_total);
                current_total = 0
            },
        }
    }

    if current_total != 0 {
        maxes.push(current_total);
    }

    maxes.sort();
    maxes.reverse();

    Some(maxes[0] + maxes[1] + maxes[2])
}

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
        assert_eq!(part_one(&input), Some(24000 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000 as u32));
    }
}
