const TOTAL_TIME: u32 = 30;

use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug)]
struct Valve {
    rate: u32,
    connected_valves: HashMap<String, u32>,
}

pub fn part_one(input: &str) -> Option<u32> {
    let valves = remove_zeros(read_input(input)?);

    let mut current_valve = "AA".to_string();
    let mut open_valves: Vec<String> = vec![];
    let mut current_time: u32 = 0;
    let mut total_flow: u32 = 0;

    loop {
        let possible_moves = get_possible_moves(&current_valve, &valves, &open_valves, TOTAL_TIME - current_time);
        if possible_moves.is_empty() {
            break;
        }
        dbg!(&possible_moves);
        
        let mut best_choice_name: &String = &"".to_string();
        let mut best_choice_return: u32 = 0;
        for (move_name, move_return) in &possible_moves {
            if *move_return > best_choice_return {
                best_choice_name = move_name;
                best_choice_return = *move_return;
            }
        }

        println!("Moving from {} to {}.", current_valve, &best_choice_name);
        current_time += valves.get(&current_valve).unwrap().connected_valves.get(best_choice_name).unwrap() + 1;
        current_valve = best_choice_name.clone();
        total_flow += best_choice_return;
        open_valves.push(current_valve.clone());
    }

    Some(total_flow)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn get_possible_moves(current_valve: &String, valves: &HashMap<String, Valve>, open_valves: &Vec<String>, time_left: u32) -> Vec<(String, u32)> {
    let mut possible_moves: Vec<(String, u32)> = vec![];
    let connected_valves = &valves.get(current_valve).unwrap().connected_valves;

    for (destination, destination_cost) in connected_valves {

        // We've already visited this node. Not a valid move.
        if open_valves.contains(destination) {
            continue;
        }

        // This choice would take too long.
        if (time_left - *destination_cost - 1) >= time_left {
            continue;
        }

        // Calculate how much flow this valve will grant us once we move there and open it
        let destination_rate = valves.get(destination).unwrap().rate;
        let total_return = (time_left - *destination_cost - 1) * destination_rate;

        possible_moves.push((destination.clone(), total_return));
    }

    possible_moves
}

fn read_input(input: &str) -> Option<HashMap<String, Valve>> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut pipes: HashMap<String, Valve> = HashMap::new();

    for line in lines {
        let line_split: Vec<&str> = line.split(' ').collect();
        let pipe_name = String::from(*line_split.get(1)?);

        let rate_word = *line_split.get(4)?;
        let rate: Vec<&str> = rate_word.split('=').collect();
        let rate = *rate.get(1)?;
        let rate: Vec<&str> = rate.split(';').collect();
        let rate = rate.first()?
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
                rate,
                connected_valves: other_valves,
            }
        );
    }

    Some(pipes)
}

fn remove_zeros(mut valves: HashMap<String, Valve>) -> HashMap<String, Valve> {

    // A vec of all the valves which have zero flow rate
    // Only includes valves which lead to other valves
    let zero_valves = find_zero_valves(&valves);

    // Find which source nodes we need to run dijkstra's on
    let mut source_nodes_without_zeros: Vec<String> = vec![];
    for source_node in valves.keys().collect::<Vec<&String>>() {
        if ! (zero_valves.contains(source_node) && **source_node != "AA".to_string()) {
            source_nodes_without_zeros.push(source_node.clone());
        }
    }

    // Each run of dijkstra's algorithm will use data from an unmodified version of valves
    let valves_backup = valves.clone();

    for source_node in source_nodes_without_zeros {
        // We are only changing the distances, so make a copy of the backup valve.
        let source_node_backup = valves_backup.get(&source_node).unwrap();
        let distances_to_other_nodes = single_valve_dijkstra(&source_node, &valves, &zero_valves);

        valves.insert(source_node, Valve {
            rate: source_node_backup.rate,
            connected_valves: distances_to_other_nodes,
        });
    }

    {
        let source_node = "AA".to_string();

        // We are only changing the distances, so make a copy of the backup valve.
        let source_node_backup = valves_backup.get(&source_node).unwrap();
        let distances_to_other_nodes = single_valve_dijkstra(&source_node, &valves, &zero_valves);

        valves.insert(source_node, Valve {
            rate: source_node_backup.rate,
            connected_valves: distances_to_other_nodes,
        });
    }

    // Remove zero valves completely
    for zero_valve in &zero_valves {
        if zero_valve != "AA" {
            valves.remove(zero_valve);
        }
    }

    valves
}

fn single_valve_dijkstra(source: &String, valves: &HashMap<String, Valve>, zero_valves: &Vec<String>) -> HashMap<String, u32> {
    let mut tentative_distances: HashMap<String, u32> = HashMap::new();
    let mut unvisited_valves: HashSet<String> = HashSet::new();

    // mark all nodes to unvisited and set initial values to infinity
    for valve_name in valves.keys() {
        unvisited_valves.insert(valve_name.clone());
        tentative_distances.insert(valve_name.clone(), u32::MAX);
    }

    // Set distance from source to source = 0
    tentative_distances.insert(source.clone(), 0);

    while ! unvisited_valves.is_empty() {
        // Find unvisited node closest to destination
        let mut closest_node: String = "".to_string();
        let mut closest_valve_distance = u32::MAX;
        for unvisited_valve in &unvisited_valves {
            if *tentative_distances.get(unvisited_valve).unwrap() < closest_valve_distance {
                closest_node = unvisited_valve.clone();
                closest_valve_distance = *tentative_distances.get(unvisited_valve).unwrap();
            }
        }

        // If there are no reachable nodes, exit.
        if closest_valve_distance == u32::MAX {
            break
        }

        // remove closest_node from unvisited nodes
        unvisited_valves.remove(&closest_node);

        // for each neighbor of closest_node still in unvisited_nodes,
        for possible_destination in &unvisited_valves {
            let possible_destination = possible_destination.clone();

            // If the graph has a path from closest_node to unvisited_neighbor,
            if let Some(neighbor_dist) = valves.get(&closest_node).unwrap().connected_valves.get(&possible_destination) {
                // Calculate alternative distance from possible_destination to destination
                let alternate_distance = tentative_distances.get(&closest_node).unwrap() + neighbor_dist;
                // Update tentative_distances if necessary
                if alternate_distance < *tentative_distances.get(&possible_destination).unwrap() {
                    tentative_distances.insert(possible_destination, alternate_distance);
                }
            }
        }
    }

    // Remove zero valves
    for zero_valve in zero_valves {
        tentative_distances.remove(zero_valve);
    }

    // Remove source valve
    tentative_distances.remove(source);

    tentative_distances
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
