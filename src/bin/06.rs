const SEQUENCE_START_LEN: usize = 4;
const MESSAGE_START_LEN: usize = 14;

pub fn part_one(input: &str) -> Option<usize> {
    find_start(input, SEQUENCE_START_LEN)
}

pub fn part_two(input: &str) -> Option<usize> {
    find_start(input, MESSAGE_START_LEN)
}

fn find_start(input: &str, start_sequence_size: usize) -> Option<usize> {
    let mut chars_read: usize;
    let mut recent_letters = vec![' '; start_sequence_size];
    let input_chars: Vec<char> = input.chars().collect();

    // init array
    for input_index in 0..start_sequence_size - 1 {
        recent_letters[input_index] = *input_chars.get(input_index).unwrap();
    }
    // init chars_read
    chars_read = start_sequence_size - 1;

    // analyze sequence
    for input_index in start_sequence_size - 1..input.len() {
        // Read next letter
        let next_letter = *input_chars.get(input_index).unwrap();
        chars_read += 1;

        // Write to recent_letters
        recent_letters[input_index % start_sequence_size] = next_letter;

        if ! contains_duplicates(&recent_letters)? {
            return Some(chars_read);
        }
    }    

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
        assert_eq!(part_one(&input), Some(10_usize));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29_usize));
    }
}
