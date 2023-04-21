use std::collections::HashSet;

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

    Some(coverage.len() as u32)
}

fn part_two_solve(input: &str) -> Option<u32> {
    part_two(input, SOLVE_UPPPER_BOUNDS)
}

pub fn part_two(input: &str, upper_bounds: i32) -> Option<u32> {
    let sensors = read_input(input)?;
    for beacon_sensor in &sensors {
        //dbg!(beacon_sensor.sensor_pos);
        let row = beacon_sensor.sensor_pos[1];
        let coverage = determine_one_row_coverage(&sensors, row);
        for i in 0..(upper_bounds+1) {
            if ! coverage.contains(&i) {
                println!("Possible coordinate found: ({}, {}). Solution: {}", i, row, ((4000000*i)+row));
            }
        }
    }
    
    None
}

fn determine_one_row_coverage(beacon_sensors: &Vec<BeaconSensor>, row: i32) -> HashSet<i32> {
    let mut coverage_cols: HashSet<i32> = HashSet::new();
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

        // We will always reach at least one cell if we get here.
        for col in (sensor_loc[0] - distance_after_reaching_row)..(sensor_loc[0] + distance_after_reaching_row) {
            coverage_cols.insert(col);
        }
    }

    coverage_cols
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
        assert_eq!(part_two(&input, TEST_UPPER_BOUND), Some(56000011u32));
    }
}
