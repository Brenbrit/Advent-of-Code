use std::{collections::{HashMap, HashSet}, cmp::min};

const MAX_STEP_UP: i32 = 1;

#[derive(Debug, Clone)]
struct Graph {
    data: HashMap<String, u32>,
    visited_nodes: HashMap<(usize, usize), Vec<(usize, usize)>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            visited_nodes: HashMap::new(),
        }
    }

    fn set(&mut self, source: (usize, usize), destination: (usize, usize), new_cost: u32) {
        match self.visited_nodes.get(&source) {
            Some(previous_list) => {
                let mut previous_list = previous_list.clone();
                previous_list.push(destination);
                self.visited_nodes.insert(source, previous_list)
            },
            None => {
                self.visited_nodes.insert(source, vec![source])
            },
        };
        self.data.insert(format!("{:?},{:?}", source, destination), new_cost);
    }

    fn get(&self, source: (usize, usize), destination: (usize, usize)) -> Option<u32> {
        // travel time from a node to itself is 0
        if source == destination {
            return Some(0)
        }

        self.data.get(&format!("{:?},{:?}", source, destination)).copied()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (topology, starting, ending) = read_input(input)?;
    let graph = generate_graph(topology)?;
    let graph = reverse_dijkstra(graph, ending)?;
    let path_size = graph.get(starting, ending)?;

    Some(path_size)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (topology, _, ending) = read_input(input)?;

    let rows = topology.len();
    let columns = topology
        .get(0)?
        .len();

    let graph = generate_graph(topology.clone())?;

    let mut min_elevation_points: Vec<(usize, usize)> = vec![];
    let mut shortest_distance_from_min = u32::MAX;

    for row in 0..rows {
        for col in 0..columns {
            if *topology.get(row).unwrap()
                .get(col).unwrap()
                == 0 {
                min_elevation_points.push((row, col));
            }
        }
    }

    let graph = reverse_dijkstra(graph, ending)?;

    for starting_point in min_elevation_points {
        match graph.get(starting_point, ending) {
            Some(dist) => {
                shortest_distance_from_min = min(shortest_distance_from_min, dist);
            },
            None => {
                continue;
            }
        };
    }

    Some(shortest_distance_from_min)
}

fn reverse_dijkstra(mut graph: Graph, destination: (usize, usize)) -> Option<Graph> {
    let mut tentative_distances: HashMap<(usize, usize), u32> = HashMap::new();
    let mut unvisited_nodes: HashSet<(usize, usize)> = HashSet::new();

    // mark all nodes to unvisited and set initial values to infinity
    for node in graph.visited_nodes.keys() {
        let node = *node;
        unvisited_nodes.insert(node);
        tentative_distances.insert(node, u32::MAX);
    }

    // Set distance from source to source = 0
    tentative_distances.insert(destination, 0);

    while ! unvisited_nodes.is_empty() {
        // Find unvisited node closest to destination
        let mut closest_node: (usize, usize) = (0, 0);
        let mut closest_node_distance = u32::MAX;
        for unvisited_node in &unvisited_nodes {
            if *tentative_distances.get(unvisited_node).unwrap() < closest_node_distance {
                closest_node = *unvisited_node;
                closest_node_distance = *tentative_distances.get(unvisited_node).unwrap();
            }
        }

        // If there are no reachable nodes, exit.
        if closest_node_distance == u32::MAX {
            break
        }

        // remove closest_node from unvisited nodes
        unvisited_nodes.remove(&closest_node);

        // for each neighbor of closest_node still in unvisited_nodes,
        for possible_source in &unvisited_nodes {
            let possible_source = *possible_source;
            // If the graph has a path from closest_node to unvisited_neighbor,
            if let Some(neighbor_dist) = graph.get(possible_source, closest_node) {
                // Calculate alternative distance from possible_source to destination
                let alternate_distance = tentative_distances.get(&closest_node).unwrap() + neighbor_dist;
                // Update tentative_distances if necessary
                if alternate_distance < *tentative_distances.get(&possible_source).unwrap() {
                    tentative_distances.insert(possible_source, alternate_distance);
                }
            }
        }
    }

    // Update graph
    for source in tentative_distances.keys() {
        let dist_from_source = *tentative_distances.get(source).unwrap();
        if dist_from_source < u32::MAX {
            graph.set(*source, destination, dist_from_source);
        }
    }

    Some(graph)
}

fn generate_graph(topology: Vec<Vec<i32>>) -> Option<Graph> {
    let mut graph = Graph::new();
    let rows = topology.len();
    let columns = topology
        .get(0)?
        .len();

    for row in 0..rows {
        for column in 0..columns {
            let tile_elevation = topology
                .get(row)
                .unwrap()
                .get(column)
                .unwrap();

            if row > 0 
            && topology
                .get(row-1)
                .unwrap()
                .get(column)
                .unwrap()
                - tile_elevation
                <= MAX_STEP_UP {
                graph.set((row, column), (row-1, column), 1);
            }

            if row < rows - 1
            && topology
                .get(row+1)
                .unwrap()
                .get(column)
                .unwrap() 
                - tile_elevation
                <= MAX_STEP_UP {
                graph.set((row, column), (row+1, column), 1);
            }

            if column > 0
            && topology
                .get(row)
                .unwrap()
                .get(column-1)
                .unwrap() 
                - tile_elevation
                <= MAX_STEP_UP {
                graph.set((row, column), (row, column-1), 1);
            }

            if column < columns - 1
            && topology
                .get(row)
                .unwrap()
                .get(column+1)
                .unwrap() 
                - tile_elevation
                <= MAX_STEP_UP {
                graph.set((row, column), (row, column+1), 1);
            }
        }
    }

    Some(graph)
}

fn read_input(input: &str) -> Option<(Vec<Vec<i32>>, (usize, usize), (usize, usize))> {
    let mut topology: Vec<Vec<i32>> = vec![];
    let mut starting: Option<(usize, usize)> = None;
    let mut ending: Option<(usize, usize)> = None;

    for (line_num, line) in input.lines().enumerate() {
        let mut row: Vec<i32> = vec![];
        for (tile_num, tile) in line.chars().enumerate() {
            if tile == 'S' {
                starting = Some((line_num, tile_num));
            } else if tile == 'E' {
                ending = Some((line_num, tile_num));
            }
            row.push(char_height(tile)?);
        }
        topology.push(row);
    }

    Some((topology, starting?, ending?))
}

fn char_height(character: char) -> Option<i32> {
    match character {
        'S' => Some(0),
        'E' => Some(25),
        character => {
            let possible_answer = (character as i32) - ('a' as i32);
            if possible_answer <= 25 {
                Some(possible_answer)
            } else {
                None
            }
        }
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31_u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29_u32));
    }
}
