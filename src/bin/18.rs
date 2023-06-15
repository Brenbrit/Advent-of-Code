use std::collections::HashSet;

const ADJACENT_COORDINATES: [[i32; 3]; 6] = [[0, 0, 1], [0, 0, -1], [0, 1, 0], [0, -1, 0], [1, 0, 0], [-1, 0, 0]];

pub fn part_one(input: &str) -> Option<u32> {
    Some(naiive_surface_area(read_input(input)?))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(shrink_wrap_surface_area(read_input(input)?))
}

// Find the surface area of the given cubes, when cubes touch. Air pockets
// are still counted.
fn naiive_surface_area(points: Vec<(u32, u32, u32)>) -> u32 {
    let mut saved_points: Vec<(u32, u32, u32)> = vec![];
    let mut surface_area: u32 = 0;

    for point in &points {
        let (x, y, z) = point;
        
        // Add point to list
        saved_points.push((*x, *y, *z));
        surface_area += 6;

        // Subtract surface area where necessary
        for adjacent_point in ADJACENT_COORDINATES {
            // Get adjacent point (could be negative - will check)
            let adjacent_x: i32 = *x as i32 + adjacent_point[0];
            let adjacent_y: i32 = *y as i32 + adjacent_point[1];
            let adjacent_z: i32 = *z as i32 + adjacent_point[2];

            // verify no negatives
            if adjacent_x < 0 || adjacent_y < 0 || adjacent_z < 0 {
                continue;
            }

            // Convert back to positive
            let adjacent_point = (adjacent_x as u32, adjacent_y as u32, adjacent_z as u32);

            // Test if we are bordering an adjacent point
            if saved_points.contains(&adjacent_point) {
                surface_area -= 2;
            }
        }
    }

    surface_area
}

fn shrink_wrap_surface_area(points: Vec<(u32, u32, u32)>) -> u32 {
    // Find shrink wrap (initially, it is just a cube around all our cubes)
    let shrink_wrap_dimensions = find_shrink_wrap_dimensions(&points);
    let mut air_bubbles: Vec<(u32, u32, u32)> = vec![];

    // Initially populate air bubbles
    for x in shrink_wrap_dimensions.0[0] + 1..shrink_wrap_dimensions.1[0] {
        for y in shrink_wrap_dimensions.0[1] + 1..shrink_wrap_dimensions.1[1] {
            for z in shrink_wrap_dimensions.0[2] + 1..shrink_wrap_dimensions.1[2] {
                if ! points.contains(&(x, y, z)) {
                    air_bubbles.push((x, y, z));
                }
            }
        }
    }

    // Filter out all exterior bubbles
    let mut interior_air_bubbles: Vec<(u32, u32, u32)> = vec![];
    for air_bubble in air_bubbles {
        if ! bubble_is_exterior(&air_bubble, &points, &shrink_wrap_dimensions) {
            interior_air_bubbles.push(air_bubble);
        }
    }

    println!("Interior air bubbles: {:?}", interior_air_bubbles);

    if interior_air_bubbles.len() > 0 {
        naiive_surface_area(points) - shrink_wrap_surface_area(interior_air_bubbles)
    } else {
        naiive_surface_area(points)
    }
}

// Given a set of points, return the dimensions of the starting shrink-wrap.
fn find_shrink_wrap_dimensions(points: &Vec<(u32, u32, u32)>) -> ([u32; 3], [u32; 3]) {
    // To form the shrink wrap efficiently, we need to know the dimensions
    // of the original surface. To do that, we need to know the min/max
    // point values.
    let mut mins: [u32; 3] = [u32::MAX; 3];
    let mut maxes: [u32; 3] = [0; 3];
    for point in points {
        let (x, y, z) = point;
        // x
        if *x < mins[0] {
            mins = [*x, mins[1], mins[2]];
        } else if *x > maxes[0] {
            maxes = [*x, maxes[1], maxes[2]];
        }
        // y
        if *y < mins[1] {
            mins = [mins[0], *y, mins[2]];
        } else if *y > maxes[1] {
            maxes = [maxes[0], *y, maxes[2]];
        }
        // z
        if *z < mins[2] {
            mins = [mins[0], mins[1], *z];
        } else if *z > maxes[2] {
            maxes = [maxes[0], maxes[1], *z];
        }
    }

    (mins, maxes)
}

// Does the bubble at the given point have a path out of the object?
fn bubble_is_exterior(bubble: &(u32, u32, u32), points: &Vec<(u32, u32, u32)>, shrink_wrap_dimensions: &([u32; 3], [u32; 3])) -> bool {
    let mut scanned_bubbles: Vec<(u32, u32, u32)> = vec![];
    let mut bubbles_to_scan: Vec<(u32, u32, u32)> = Vec::from([*bubble]);
    while ! &bubbles_to_scan.is_empty() {
        let mut next_bubbles_to_scan: Vec<(u32, u32, u32)> = vec![];
        for bubble in &bubbles_to_scan {
            // Have we reached the outside?
            if ! point_in_shrink_wrap(&bubble, shrink_wrap_dimensions) {
                return true;
            }

            // This bubble is not *currently* exterior - it is scanned.
            scanned_bubbles.push(bubble.clone());

            // We have not reached the outside. Add all bubbles we can reach.
            for adjacent_point in ADJACENT_COORDINATES {
                // Get adjacent point (could be negative - will check)
                let adjacent_x: i32 = bubble.0 as i32 + adjacent_point[0];
                let adjacent_y: i32 = bubble.1 as i32 + adjacent_point[1];
                let adjacent_z: i32 = bubble.2 as i32 + adjacent_point[2];

                // verify no negatives, duplicates, or cube-points
                if adjacent_x < 0 || adjacent_y < 0 || adjacent_z < 0 {
                    continue;
                }

                // Convert back to positive
                let adjacent_point = (adjacent_x as u32, adjacent_y as u32, adjacent_z as u32);

                // Add to next_bubbles_to_scan if possible
                if ! points.contains(&adjacent_point) 
                && ! scanned_bubbles.contains(&adjacent_point) 
                && ! next_bubbles_to_scan.contains(&adjacent_point) {
                    next_bubbles_to_scan.push(adjacent_point);
                }
            }
        }

        bubbles_to_scan = next_bubbles_to_scan;
    }

    false
}

// Is a given coordinate within the shrink-wrap?
fn point_in_shrink_wrap(point: &(u32, u32, u32), shrink_wrap_dimensions: &([u32; 3], [u32; 3])) -> bool {
    let (point_x, point_y, point_z) = *point;
    let (mins, maxes) = shrink_wrap_dimensions;

    point_x > mins[0] && point_x < maxes[0]
    && point_y > mins[1] && point_y < maxes[1]
    && point_z > mins[1] && point_z < maxes[2]
}

/*// Given two points which are opposite vertices of the dimensions of the
// points, form an initial shrink-wrap. Currently, this approach uses
// simple for-loops and a HashSet, but could possibly be improved by re-writing
// the for-loops and checking for duplicate bounds.
fn form_shrink_wrap(shrink_wrap_dimensions: ([u32; 3], [u32; 3])) -> HashSet<(u32, u32, u32)> {
    let (mins, maxes) = shrink_wrap_dimensions;
    let mut shrink_wrap: HashSet<(u32, u32, u32)> = HashSet::new();

    // start with both x-y planes
    for x in mins[0]..maxes[0] {
        for y in mins[1]..maxes[1] {
            // Add a point for both min and max z (assuming they are different)
            shrink_wrap.insert((x, y, mins[2]));
            shrink_wrap.insert((x, y, maxes[2]));
        }
    }

    // both x-z planes
    for x in mins[0]..maxes[0] {
        for z in mins[2]..maxes[2] {
            // Add a point for both min and max z (assuming they are different)
            shrink_wrap.insert((x, mins[1], z));
            shrink_wrap.insert((x, maxes[1], z));
        }
    }

    // both y-z planes
    for y in mins[1]..maxes[1] {
        for z in mins[2]..maxes[2] {
            // Add a point for both min and max z (assuming they are different)
            shrink_wrap.insert((mins[0], y, z));
            shrink_wrap.insert((maxes[0], y, z));
        }
    }

    shrink_wrap
}

// If this function returns false, then it is possible for the given 
// shrink-wrap to become smaller.
fn shrink_wrap_is_tight(shrink_wrap: &HashSet<(u32, u32, u32)>, points: &Vec<(u32, u32, u32)>) -> bool {
    shrink_wrap_only_air(shrink_wrap, points).len() == 0
}

// Filters out the points of the shrink wrap which overlap cubes.
fn shrink_wrap_only_air(shrink_wrap: &HashSet<(u32, u32, u32)>, points: &Vec<(u32, u32, u32)>) -> HashSet<(u32, u32, u32)> {
    let mut only_air: HashSet<(u32, u32, u32)> = HashSet::new();

    for shrink_wrap_point in shrink_wrap {
        if ! points.contains(shrink_wrap_point) {
            only_air.insert(*shrink_wrap_point);
        }
    }

    only_air
}
*/

// Given input, return a list of points in the form of Vec<(u32, u32, u32)>
fn read_input(input: &str) -> Option<Vec<(u32, u32, u32)>> {
    let mut points: Vec<(u32, u32, u32)> = vec![];

    for point_str in input.lines() {
        // Interpret line as coordinates in x,y,z
        let line_split: Vec<&str> = point_str.trim().split(',').collect();
        let x = line_split.get(0)?.parse::<u32>().unwrap();
        let y = line_split.get(1)?.parse::<u32>().unwrap();
        let z = line_split.get(2)?.parse::<u32>().unwrap();
        
        // Add point to list
        points.push((x, y, z));
    }

    Some(points)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64_u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58_u32));
    }
}
