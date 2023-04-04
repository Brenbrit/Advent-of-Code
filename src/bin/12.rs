use std::{collections::{HashMap, HashSet}, time::Instant};

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

        match self.data.get(&format!("{:?},{:?}", source, destination)) {
            Some(i) => Some(*i),
            None => None,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (topology, starting, ending) = read_input(input)?;
    let graph = generate_graph(topology)?;
    let (_, path_size) = dijkstra_shortest_path(graph, starting, ending)?;

    Some(path_size)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (topology, _, ending) = read_input(input)?;

    let rows = topology.len();
    let columns = topology
        .get(0)?
        .len();

    let mut graph = generate_graph(topology.clone())?;

    let mut min_elevation_points: Vec<(usize, usize)> = vec![];
    let mut shortest_distance_from_min = u32::MAX;

    for row in 0..rows {
        for col in 0..columns {
            if topology.get(row).unwrap()
                .get(col).unwrap()
                .clone()
                == 0 {
                min_elevation_points.push((row, col));
            }
        }
    }

    // Time this function
    // it takes a long time.
    let start_time = Instant::now();

    for i in 0..min_elevation_points.len() {
        let min_elevation_point = min_elevation_points.get(i).unwrap();
        let (new_graph, path_size) = dijkstra_shortest_path(graph, min_elevation_point.clone(), ending)?;
        if path_size < shortest_distance_from_min {
            shortest_distance_from_min = path_size;
        }
        graph = new_graph;

        println!("{}/{} ({:.2}%). Expected remaining time: {:.2} seconds.", 
            i+1, 
            min_elevation_points.len(), 
            f64::from(((i+1)*100) as u32) / f64::from(min_elevation_points.len() as u32),
            (start_time.elapsed().as_secs_f64() / f64::from((i as u32)+1)) * f64::from((min_elevation_points.len()-i) as u32),
        );
    }

    Some(shortest_distance_from_min)
}

fn dijkstra_shortest_path(mut graph: Graph, starting: (usize, usize), ending: (usize, usize)) -> Option<(Graph, u32)> {

    let mut tentative_distances: HashMap<(usize, usize), u32> = HashMap::new();
    let mut unvisited_nodes: HashSet<(usize, usize)> = HashSet::new();

    // mark all nodes to unvisited and set initial values to infinity
    for node in graph.visited_nodes.keys() {
        let node = node.clone();
        unvisited_nodes.insert(node);
        tentative_distances.insert(node, u32::MAX);
    }

    // Set distance from source to source = 0
    tentative_distances.insert(starting, 0);

    while ! unvisited_nodes.is_empty() {
        // Find unvisited node closest to starting
        let mut closest_node: (usize, usize) = (0, 0);
        let mut closest_node_distance = u32::MAX;
        for unvisited_node in &unvisited_nodes {
            if *tentative_distances.get(unvisited_node).unwrap() < closest_node_distance {
                closest_node = unvisited_node.clone();
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
        for possible_neighbor in &unvisited_nodes {
            let possible_neighbor = possible_neighbor.clone();
            // If the graph has a path from closest_node to unvisited_neighbor,
            if let Some(neighbor_dist) = graph.get(closest_node, possible_neighbor) {
                // Calculate alternative distance from source to possible_neighbor
                let alternate_distance = tentative_distances.get(&closest_node).unwrap() + neighbor_dist;
                // Update tentative_distances if necessary
                if alternate_distance < *tentative_distances.get(&possible_neighbor).unwrap() {
                    tentative_distances.insert(possible_neighbor, alternate_distance);
                }
            }
        }
    }

    // Update graph
    for destination_node in tentative_distances.keys() {
        graph.set(starting, *destination_node, *tentative_distances.get(destination_node).unwrap());
    }

    let shortest_path_cost = graph.get(starting, ending)?;
    Some((graph, shortest_path_cost))
}

fn reverse_dijkstra(graph: Graph, destination: (usize, usize)) -> Option<Graph> {
    let mut tentative_distances: HashMap<(usize, usize), u32> = HashMap::new();
    let mut unvisited_nodes: HashSet<(usize, usize)> = HashSet::new();

    // mark all nodes to unvisited and set initial values to infinity
    for node in graph.visited_nodes.keys() {
        let node = node.clone();
        unvisited_nodes.insert(node);
        tentative_distances.insert(node, u32::MAX);
    }

    // Set distance from source to source = 0
    tentative_distances.insert(destination, 0);

    while ! unvisited_nodes.is_empty() {
        // Find unvisited node closest to starting
        let mut closest_node: (usize, usize) = (0, 0);
        let mut closest_node_distance = u32::MAX;
        for unvisited_node in &unvisited_nodes {
            if *tentative_distances.get(unvisited_node).unwrap() < closest_node_distance {
                closest_node = unvisited_node.clone();
                closest_node_distance = *tentative_distances.get(unvisited_node).unwrap();
            }
        }
    }

    Some((graph))
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
        assert_eq!(part_one(&input), Some(31 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29 as u32));
    }
}
