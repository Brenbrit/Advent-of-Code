const SEQUENCE_START_LEN: usize = 4;

pub fn part_one(input: &str) -> Option<u32> {
    let mut chars_read: u32;
    let mut recent_letters = [' '; SEQUENCE_START_LEN];
    let input_chars: Vec<char> = input.chars().collect();

    // init array
    for input_index in 0..SEQUENCE_START_LEN - 1 {
        recent_letters[input_index] = *input_chars.get(input_index).unwrap();
    }
    // init chars_read
    chars_read = (SEQUENCE_START_LEN - 1) as u32;

    // analyze sequence
    for input_index in SEQUENCE_START_LEN - 1..input.len() {
        // Read next letter
        let next_letter = *input_chars.get(input_index).unwrap();
        chars_read += 1;

        // Write to recent_letters
        recent_letters[input_index % SEQUENCE_START_LEN] = next_letter;

        if ! contains_duplicates(&recent_letters)? {
            return Some(chars_read);
        }
    }    

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn contains_duplicates(letters: &[char]) -> Option<bool> {

    for index_1 in 0..letters.len() {
        for index_2 in index_1+1..letters.len() {
            if letters[index_1] == letters[index_2] {
                return Some(true);
            }
        }
    }

    Some(false)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
