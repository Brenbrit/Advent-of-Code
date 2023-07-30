pub fn part_one(input: &str) -> Option<i32> {
    let numbers = read_input(input).expect("Failed to parse input");
    let mixables = convert_to_mixable_vec(numbers);

    /*print!("Initial arrangement: [");
    for m in &mixables {
        print!("{}, ", m.number);
    }
    println!("]\n");*/

    let mixed = mix(mixables);

    // We need to know the position of the zero for the next step!
    let zero_position = mixed.iter().position(|&z| z == Mixable{mixed: true, number: 0}).unwrap();
    
    /*print!("Mixed: [");
    for m in &mixed {
        print!("{}, ", m.number);
    }
    println!("]");
    println!("Zero is at position {}", zero_position);*/

    let mut coordinate_sum: i32 = 0;

    for coordinate_zero_offset in [1000_usize, 2000, 3000] {
        let coordinate_index = (zero_position + coordinate_zero_offset) % mixed.len();
        coordinate_sum += mixed.get(coordinate_index).unwrap().number;
    }

    Some(coordinate_sum)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Mixable {
    mixed: bool,
    number: i32,
}

fn mix(mut numbers: Vec<Mixable>) -> Vec<Mixable> {

    let numbers_len = numbers.len() as i32;

    loop {
        let next_unmixed_index = match find_first_unmixed_index(&numbers) {
            Some(i) => i,
            None => break,
        };

        let number = numbers.remove(next_unmixed_index).number;

        let mut negative_offset = 0;
        if number < 0 {
            negative_offset = -1;
        } else if (number + numbers_len - 1) > numbers_len {
            negative_offset = 1;
        }

        // To calculate the next position, we need to add the current position to the number.
        // To account for the wrap-around, we add numbers_len and then modulo numbers_len.
        //println!("Next unmixed index: {}. as i32: {}.", next_unmixed_index, next_unmixed_index as i32);
        //dbg!(((next_unmixed_index as i32) + number + numbers_len + negative_offset));
        
        let next_position = (((next_unmixed_index as i32) + number + numbers_len + numbers_len + negative_offset) % numbers_len) as usize;

        // println!("{} moves between {} and {}.", number, numbers.get((next_position - 1 + numbers.len()) % numbers.len()).unwrap().number, numbers.get(next_position % numbers.len()).unwrap().number);

        numbers.insert(next_position, Mixable {
            mixed: true,
            number,
        });

        /*print!("list after this move: [");
        for m in &numbers {
            print!("{}, ", m.number);
        }
        println!("]\n");*/
    }
    
    numbers
}

fn find_first_unmixed_index(numbers: &Vec<Mixable>) -> Option<usize> {
    for i in 0..numbers.len() {
        if ! numbers.get(i).unwrap().mixed {
            return Some(i);
        }
    }

    // all numbers were mixed.
    None
}

fn convert_to_mixable_vec(numbers: Vec<i32>) -> Vec<Mixable> {
    let mut mixables: Vec<Mixable> = vec![];
    for number in numbers {
        mixables.push(Mixable{
            mixed: false,
            number,
        });
    }

    mixables
}

fn read_input(input: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    let lines = input.lines().collect::<Vec<&str>>();

    // Add each parsed number to the numbers vector
    let mut numbers: Vec<i32> = vec![];
    for line in lines {
        // propagate error up the chain if found
        numbers.push(line.parse::<i32>()?);
    }

    Ok(numbers)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3_i32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
