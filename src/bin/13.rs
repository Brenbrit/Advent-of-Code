#[derive(Debug)]
struct List {
    elements: Vec<ListElement>,
}

impl From<&str> for List {
    fn from(string: &str) -> Self {
        println!("Interpreting as list: '{}'", string);
        let chars = string.chars().collect::<Vec<char>>();
        let last_char_index = List::list_len(&chars).unwrap();
        println!("This list is from index {} to {}.", 0, last_char_index);

        let mut elements: Vec<ListElement> = vec![];
        let mut current_index: usize = 1;
        while current_index < last_char_index {
            println!("Current char: {}", *chars.get(current_index).unwrap());
            match *chars.get(current_index).unwrap() {
                '[' => {
                    // we have a nested list
                    let nested_list_last_char = List::list_len(&chars[current_index..])
                        .unwrap();
                    println!("Nested list runs from {} to {}", current_index, nested_list_last_char);
                    let nested_list = List::from(&string[current_index..nested_list_last_char]);
                    elements.push(ListElement::List(nested_list));
                    current_index = nested_list_last_char;
                },
                ',' => { current_index += 1 },
                ']' => { current_index += 1 },
                _ => {
                    // we are at a number
                    let rest_of_string: String = chars[current_index..].iter().collect();
                    let end_of_num_index = rest_of_string
                        .chars()
                        .position(|c| c == ',' || c == ']')
                        .unwrap();
                    println!("Interpreting as number: '{}'", &rest_of_string[..end_of_num_index]);
                    current_index += end_of_num_index;
                }
            }
        }

        List { elements: vec![] }
    }
}

impl List {
    // Assumes that the first char is the leading [ of the list.
    fn list_len(chars: &[char]) -> Option<usize> {
        let mut last_char_index: usize = 0;
        let mut num_nested_lists: u32 = 1;
        for i in 1..chars.len() {
            match chars.get(i).unwrap() {
                '[' => { num_nested_lists += 1 },
                ']' => {
                    num_nested_lists -= 1;
                    if num_nested_lists == 0 {
                        last_char_index = i;
                        break;
                    }
                },
                _ => {},
            }
        }
        
        match last_char_index {
            0 => None,
            index => Some(index)
        }
    }
}

#[derive(Debug)]
enum ListElement {
    Number(u32),
    List(List),
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = read_input(input)?;
    //dbg!(&pairs);

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn read_input(input: &str) -> Option<Vec<[List; 2]>> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut lists: Vec<[List; 2]> = vec![];

    for line_group in lines.chunks(3) {
        if line_group.len() < 2 {
            println!("Last group of lists is incomplete - not interpreting.");
            println!("Problem-causing group: {:?}", line_group);
            continue;
        }
        let first_list = List::from(line_group[0]);
        let second_list = List::from(line_group[1]);
        lists.push([first_list, second_list]);
    }

    Some(lists)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}
