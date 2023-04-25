use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Valve {
    rate: u32,
    connected_valves: Vec<String>,
    open: bool,
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = read_input(input)?;
    dbg!(valves);
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn read_input(input: &str) -> Option<HashMap<String, Valve>> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut pipes: HashMap<String, Valve> = HashMap::new();

    for line in lines {
        let line_split: Vec<&str> = line.split(" ").collect();
        let pipe_name = String::from(*line_split.get(1)?);

        let rate_word = *line_split.get(4)?;
        let rate: Vec<&str> = rate_word.split("=").collect();
        let rate = *rate.get(1)?;
        let rate: Vec<&str> = rate.split(";").collect();
        let rate = rate
            .get(0)?
            .parse::<u32>()
            .unwrap();

        let mut other_valves: Vec<String> = vec![];
        for other_valve in line_split.iter().skip(9) {
            let other_valve = String::from(&other_valve[..2]);
            other_valves.push(other_valve);
        }

        pipes.insert(
            pipe_name,
            Valve {
                rate: rate,
                connected_valves: other_valves,
                open: false,
            }
        );
    }

    Some(pipes)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}
