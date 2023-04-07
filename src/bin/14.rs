use std::{collections::HashSet, cmp::max};

const INITIAL_DROP_LOCATION: [u32; 2] = [500, 0];

pub fn part_one(input: &str) -> Option<u32> {
    let (solid_tiles, lowest_rock) = read_rock_formations(input)?;
    Some(count_dropped_sand(solid_tiles, lowest_rock, INITIAL_DROP_LOCATION, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    let (solid_tiles, lowest_rock) = read_rock_formations(input)?;
    Some(count_dropped_sand(solid_tiles, 2 + lowest_rock, INITIAL_DROP_LOCATION, true))
}

fn count_dropped_sand(
    mut solid_tiles: HashSet<[u32; 2]>, 
    lowest_rock: u32, 
    initial_drop_location: [u32; 2],
    solid_floor: bool
    ) -> u32 {
    let mut dropped_sand = 0;
    let mut current_position: [u32; 2];
    
    loop {
        current_position = initial_drop_location;

        loop {
            if current_position[1] >= lowest_rock {
                return dropped_sand
            }

            // try falling straight down
            match solid_tiles.get(&[current_position[0], current_position[1] + 1]) {
                Some(_) => {},
                None => {
                    if !solid_floor || current_position[1] + 1 < lowest_rock {
                        current_position[1] += 1;
                        continue;
                    }
                }
            }

            // try falling left
            match solid_tiles.get(&[current_position[0] - 1, current_position[1] + 1]) {
                Some(_) => {},
                None => {
                    if !solid_floor || current_position[1] + 1 < lowest_rock {
                        current_position[0] -= 1;
                        current_position[1] += 1;
                        continue;
                    }
                }
            }

            // try falling right
            match solid_tiles.get(&[current_position[0] + 1, current_position[1] + 1]) {
                Some(_) => {},
                None => {
                    if !solid_floor || current_position[1] + 1 < lowest_rock {
                        current_position[0] += 1;
                        current_position[1] += 1;
                        continue;
                    }
                }
            }

            // The sand did not move this turn - it is stuck!
            solid_tiles.insert(current_position);
            dropped_sand += 1;
            break;
        }

        if current_position == initial_drop_location {
            break;
        }
    }

    dropped_sand
}

fn read_rock_formations(input: &str) -> Option<(HashSet<[u32; 2]>, u32)> {
    let mut formations: HashSet<[u32; 2]> = HashSet::new();
    let mut lowest_rock: u32 = 0;

    for line in input.lines() {
        let rock_vertices: Vec<&str> = line.split(" -> ").collect();
        if rock_vertices.len() == 0 { continue }

        // Read first vertex of formation and add to formations
        let first_vertex = parse_rock_coordinates(rock_vertices.first().unwrap())?;
        formations.insert(first_vertex);
        // update lowest_rock if necessary
        lowest_rock = max(lowest_rock, first_vertex[1]);

        let mut last_rock_tile = first_vertex;

        // Rest of formationsmove
        for vertex in rock_vertices.iter().skip(1) {
            let vertex = parse_rock_coordinates(vertex)?;
            lowest_rock = max(lowest_rock, vertex[1]);
            while vertex != last_rock_tile {
                // Get location of next rock
                if vertex[0] < last_rock_tile[0] {
                    // we need to move to the left
                    last_rock_tile[0] -= 1;
                } else if vertex[0] > last_rock_tile[0] {
                    // we need to move to the right
                    last_rock_tile[0] += 1;
                } else if vertex[1] < last_rock_tile[1] {
                    // we need to move down
                    last_rock_tile[1] -= 1;
                } else {
                    // we need to move up
                    last_rock_tile[1] += 1;
                }

                // save new rock to list
                formations.insert(last_rock_tile);
            }

            // Update last_rock_tile (sanity check)
            last_rock_tile = vertex;
        }
    }
    
    Some((formations, lowest_rock))
}

fn parse_rock_coordinates(coordinates: &str) -> Option<[u32; 2]> {
    let split: Vec<&str> = coordinates.split(",").collect();
    Some(
        [split.get(0)?.parse::<u32>()
        .expect("Failed to parse X value of coordinate"), 
        split.get(1)?.parse::<u32>()
        .expect("Failed to parse Y value of coordinate")
    ])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93u32));
    }
}
