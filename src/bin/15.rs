use std::collections::HashSet;

// For debugging and testing: 10
// For solving: 2000000
const ROW: i32 = 10;

#[derive(Debug, Clone)]
struct BeaconSensor {
    sensor_pos: [i32; 2],
    nearest_beacon: [i32; 2],
}

pub fn part_one(input: &str) -> Option<u32> {
    println!("Reading sensors");
    let sensors = read_input(input)?;    
    println!("Determining coverage");
    let coverage = determine_coverage(sensors);
    println!("Solving problem");
    let covered_cols = get_row_coverage(&coverage, ROW);

    // Debugging!
    let mut covered_cols_vec = vec![];
    for col in &covered_cols {
        covered_cols_vec.push(*col);
    }
    covered_cols_vec.sort();
    println!("{:?}", covered_cols_vec);
    println!("len: {}", covered_cols_vec.len());

    Some(covered_cols.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn get_row_coverage(coverage: &HashSet<(i32, i32)>, row: i32) -> HashSet<i32> {
    let mut covered_columns: HashSet<i32> = HashSet::new();
    for (x, y) in coverage {
        if *y == row {
            covered_columns.insert(*x);
        }
    }

    covered_columns
}

fn determine_coverage(beacon_sensors: Vec<BeaconSensor>) -> HashSet<(i32, i32)> {
    let mut coverage_areas: HashSet<(i32, i32)> = HashSet::new();
    let mut current_beacon = 1;

    for beacon_sensor in &beacon_sensors {
        println!("  ==> {}/{}", current_beacon, beacon_sensors.len());
        current_beacon += 1;

        let sensor_loc = beacon_sensor.sensor_pos;
        let beacon_loc = beacon_sensor.nearest_beacon;

        let manhattan_distance = (sensor_loc[0] - beacon_loc[0]).abs()
            + (sensor_loc[1] - beacon_loc[1]).abs();
        for manhattan_distance in 1..(manhattan_distance + 1) {
            // northernmost covered spot
            let mut covered_spot = [sensor_loc[0], sensor_loc[1] + manhattan_distance];

            // move southeast
            while covered_spot != [sensor_loc[0] + manhattan_distance, sensor_loc[1]] {
                coverage_areas.insert((covered_spot[0], covered_spot[1]));
                covered_spot[0] += 1;
                covered_spot[1] -= 1;
            }

            // move southwest
            while covered_spot != [sensor_loc[0], sensor_loc[1] - manhattan_distance] {
                coverage_areas.insert((covered_spot[0], covered_spot[1]));
                covered_spot[0] -= 1;
                covered_spot[1] -= 1;
            }

            // move northwest
            while covered_spot != [sensor_loc[0] - manhattan_distance, sensor_loc[1]] {
                coverage_areas.insert((covered_spot[0], covered_spot[1]));
                covered_spot[0] -= 1;
                covered_spot[1] += 1;
            }

            // move northeast
            while covered_spot != [sensor_loc[0], sensor_loc[1] + manhattan_distance] {
                coverage_areas.insert((covered_spot[0], covered_spot[1]));
                covered_spot[0] += 1;
                covered_spot[1] += 1;
            }
        }
    }

    // The covered areas do not include previously-discovered beacons.
    for beacon_sensor in &beacon_sensors {
        coverage_areas.remove(&(beacon_sensor.nearest_beacon[0], beacon_sensor.nearest_beacon[0]));
    }

    coverage_areas
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
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
