const SOLVE_ROW: i32 = 2000000;
const SOLVE_UPPPER_BOUNDS: i32 = 4000000;

#[derive(Debug, Clone)]
struct BeaconSensor {
    sensor_pos: [i32; 2],
    nearest_beacon: [i32; 2],
}

fn part_one_solve(input: &str) -> Option<u32> {
    part_one(input, SOLVE_ROW)
}

pub fn part_one(input: &str, row: i32) -> Option<u32> {
    let sensors = read_input(input)?;
    let coverage = determine_one_row_coverage(&sensors, row);

    let mut total_coverage: u32 = 0;
    for area in coverage {
        total_coverage += (area[1] - area[0]) as u32;
    }

    Some(total_coverage as u32)
}

fn part_two_solve(input: &str) -> Option<u64> {
    part_two(input, SOLVE_UPPPER_BOUNDS)
}

pub fn part_two(input: &str, upper_bounds: i32) -> Option<u64> {
    let sensors = read_input(input)?;
    for row in 0..upper_bounds {
        let coverage = determine_one_row_coverage(&sensors, row);
        let coverage = limit_coverage(coverage, 0, upper_bounds);
        
        if coverage.len() != 1 {
            let x = (coverage.get(0).unwrap()[1] + 1) as u64;
            let y = row as u64;
            return Some((4000000 * x) + y);
        }
    }
    
    None
}

// Returns a Vec<[i32; 2]> where each element is the minimun and maximum
// covered tile.
fn determine_one_row_coverage(beacon_sensors: &Vec<BeaconSensor>, row: i32) -> Vec<[i32; 2]> {
    let mut coverage_sets: Vec<[i32; 2]> = vec![];
    //let mut current_beacon = 1;

    for beacon_sensor in beacon_sensors {
        //println!("  ==> {}/{}", current_beacon, beacon_sensors.len());
        //current_beacon += 1;

        let sensor_loc = beacon_sensor.sensor_pos;
        let beacon_loc = beacon_sensor.nearest_beacon;

        let manhattan_distance = (sensor_loc[0] - beacon_loc[0]).abs()
            + (sensor_loc[1] - beacon_loc[1]).abs();
        
        let vertical_distance_from_row = (sensor_loc[1] - row).abs();

        // We are too far from the row - no coverage.
        if vertical_distance_from_row > manhattan_distance {
            continue
        }

        let distance_after_reaching_row = manhattan_distance - vertical_distance_from_row;

        let lower_bound = sensor_loc[0] - distance_after_reaching_row;
        let upper_bound = sensor_loc[0] + distance_after_reaching_row;

        coverage_sets.push([lower_bound, upper_bound]);
    }

    consolidate_coverage(coverage_sets)
}

fn consolidate_coverage(coverage_raw: Vec<[i32; 2]>) -> Vec<[i32; 2]> {
    let mut consolidated: Vec<[i32; 2]> = vec![];

    'sensor: for sensor_coverage in coverage_raw {

        for previous_consolidated in &consolidated {
            // Is this sensor completely inside another?
            if sensor_coverage[0] >= previous_consolidated[0]
            && sensor_coverage[1] <= previous_consolidated[1] {
                continue 'sensor;
            }

            // Is another sensor completely inside this one?
            if sensor_coverage[0] <= previous_consolidated[0]
            && sensor_coverage[1] >= previous_consolidated[1] {
                // Remove old sensor
                consolidated.remove(consolidated.iter().position(|&r| r == *previous_consolidated).unwrap());
                // Add new sensor
                consolidated.push(sensor_coverage);
                // Sort vec
                consolidated.sort();
                continue 'sensor;
            }
        }

        consolidated.push(sensor_coverage);
        // Sort list
        consolidated.sort();
    }

    // Actually consolidate each item in consolidated
    let mut i: usize = 0;
    while i < (consolidated.len() - 1) {
        let current_area = consolidated.get(i).unwrap();
        let next_area = consolidated.get(i+1).unwrap();

        if current_area[1] >= next_area[0] {
            let new_area = [current_area[0], std::cmp::max(current_area[1], next_area[1])];
            consolidated.remove(i);
            consolidated.remove(i);
            consolidated.insert(i, new_area);
        } else if current_area[1] + 1 == next_area[0] {
            let new_area = [current_area[0], next_area[1]];
            consolidated.remove(i);
            consolidated.remove(i);
            consolidated.insert(i, new_area);
        } else {
            i += 1;
        }
    }

    consolidated
}

// Limits coverage to a minimum of min and a maximum of max
// Also removes any coverage areas which are completely outside of that limit
fn limit_coverage(coverage: Vec<[i32; 2]>, min: i32, max: i32) -> Vec<[i32; 2]> {
    let mut limited: Vec<[i32; 2]> = vec![];

    for area in coverage {

        // Does this engulf the entire limit?
        if area[0] <= min && area[1] >= max {
            return vec![[min, max]];
        }

        // Does the upper bound fall within the limit?
        if area[1] >= min && area[1] <= max {
            limited.push([std::cmp::max(area[0], min), area[1]]);
            continue;
        }
        // Does the lower bound fall within the limit?
        if area[0] >= min && area[0] <= max {
            limited.push([area[0], std::cmp::min(area[1], max)]);
            continue;
        }
    }

    limited
}

fn read_input(input: &str) -> Option<Vec<BeaconSensor>> {
    let mut beacons: Vec<BeaconSensor> = vec![];

    for line in input.lines() {
        let sensor_x: Vec<&str> = line.split("x=").collect();
        let sensor_x: Vec<&str> = sensor_x.get(1)?.split(",").collect();
        let sensor_x = sensor_x
            .get(0)?
            .parse::<i32>()
            .expect("Failed to parse sensor x position");

        let sensor_y: Vec<&str> = line.split("y=").collect();
        let sensor_y: Vec<&str> = sensor_y.get(1)?.split(":").collect();
        let sensor_y = sensor_y
            .get(0)?
            .parse::<i32>()
            .expect("Failed to parse sensor y position");

        let beacon_x: Vec<&str> = line.split("x=").collect();
        let beacon_x: Vec<&str> = beacon_x.get(2)?.split(",").collect();
        let beacon_x = beacon_x
            .get(0)?
            .parse::<i32>()
            .expect("Failed to parse beacon x position");

        let beacon_y: Vec<&str> = line.split("y=").collect();
        let beacon_y = beacon_y
            .get(2)?
            .parse::<i32>()
            .expect("Failed to parse beacon x position");

        beacons.push(
            BeaconSensor { sensor_pos: [sensor_x, sensor_y], nearest_beacon: [beacon_x, beacon_y] }
        );
    }

    Some(beacons)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one_solve, input);
    advent_of_code::solve!(2, part_two_solve, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_ROW: i32 = 10;
    const TEST_UPPER_BOUND: i32 = 20;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input, TEST_ROW), Some(26u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input, TEST_UPPER_BOUND), Some(56000011u64));
    }
}
