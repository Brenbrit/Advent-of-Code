use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Valve {
    rate: u32,
    connected_valves: HashMap<String, u32>,
    open: bool,
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = remove_zeros(read_input(input)?, 10);
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

        let mut other_valves: HashMap<String, u32> = HashMap::new();
        for other_valve in line_split.iter().skip(9) {
            let other_valve = String::from(&other_valve[..2]);
            other_valves.insert(other_valve, 1);
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

fn remove_zeros(mut valves: HashMap<String, Valve>) -> HashMap<String, Valve> {

    // A vec of all the valves which have zero flow rate
    // Only includes valves which lead to other valves
    let zero_valves = find_zero_valves(&valves);

    let mut source_nodes: Vec<&String> = valves.keys().collect();
    for (index, source_node) in (&source_nodes).iter().enumerate() {
        if zero_valves.contains(source_node) && **source_node != "AA".to_string() {
            source_nodes.remove(index);
        }
    }

    valves
}

fn find_zero_valves(valves: &HashMap<String, Valve>) -> Vec<String> {
    // A HashSet of all the valves which have zero flow rate
    // Only includes valves which lead to other valves
    let mut zero_valves: HashSet<String> = HashSet::new();
    for valve in valves.keys() {
        if valves.get(valve).unwrap().rate == 0 {
            for other_valve in valves.keys() {
                //dbg!(&valves.get(valve).unwrap().connected_valves);
                for (connected_valve, _) in &valves.get(other_valve).unwrap().connected_valves {
                    if *connected_valve == *valve {
                        // other_valve leads to valve, which has a 0 flow rate
                        zero_valves.insert(valve.clone());
                    }
                }
            }
        }
    }

    let mut zero_valves_vec: Vec<String> = vec![];

    for valve in zero_valves {
        zero_valves_vec.push(valve);
    }

    zero_valves_vec
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
