use std::collections::HashMap;

const MAX_STEP_UP: i32 = 1;

#[derive(Debug)]
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
    dbg!(&topology, &starting, &ending);
    let graph = generate_graph(topology)?;
    dbg!(&graph);
    let (_, path_size) = dijkstra_shortest_path(graph, starting, ending)?;

    Some(path_size)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn dijkstra_shortest_path(graph: Graph, starting: (usize, usize), ending: (usize, usize)) -> Option<(Graph, u32)> {

    // TODO: implement Dijkstra's algo

    let shortest_path_cost = graph.get(starting, ending)?;
    Some((graph, shortest_path_cost))
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
            && tile_elevation - topology
                .get(row-1)
                .unwrap()
                .get(column)
                .unwrap()
                <= MAX_STEP_UP {
                graph.set((row, column), (row-1, column), 1);
            }

            if row < rows - 1
            && tile_elevation - topology
                .get(row+1)
                .unwrap()
                .get(column)
                .unwrap() 
                <= MAX_STEP_UP {
                graph.set((row, column), (row+1, column), 1);
            }

            if column > 0
            && tile_elevation - topology
                .get(row)
                .unwrap()
                .get(column-1)
                .unwrap() 
                <= MAX_STEP_UP {
                graph.set((row, column), (row, column-1), 1);
            }

            if column < columns - 1
            && tile_elevation - topology
                .get(row)
                .unwrap()
                .get(column+1)
                .unwrap() 
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
        assert_eq!(part_two(&input), None);
    }
}
