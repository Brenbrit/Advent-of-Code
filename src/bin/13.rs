#[derive(Debug)]
struct List {
    elements: Vec<ListElement>,
}

impl From<&str> for List {
    fn from(string: &str) -> Self {
        let chars = string.chars().collect::<Vec<char>>();
        let last_char_index = List::list_len(&chars).unwrap();

        let mut elements: Vec<ListElement> = vec![];
        let mut current_index: usize = 1;
        while current_index < last_char_index {
            match *chars.get(current_index).unwrap() {
                '[' => {
                    // we have a nested list
                    let nested_list_last_char = List::list_len(&chars[current_index..])
                        .unwrap() + current_index + 1;
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
                    elements.push(ListElement::Number(rest_of_string[..end_of_num_index].parse::<u32>().unwrap()));
                    current_index += end_of_num_index;
                }
            }
        }

        List { elements: elements }
    }
}

impl List {
    // Assumes that the first char is the leading [ of the list.
    fn list_len(chars: &[char]) -> Option<usize> {
        let mut last_char_index: usize = 0;
        let mut num_nested_lists: u32 = 0;
        for i in 0..chars.len() {
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

#[derive(Debug)]
enum ListOrder {
    LeftFirst,
    RightFirst,
    Equal,
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs = read_input(input)?;
    
    let mut result = 0;
    for i in 0..pairs.len() {
        let pair = pairs.get(i).unwrap();
        let list_order = compare_lists(&pair[0], &pair[1]);
        match list_order {
            ListOrder::LeftFirst => { result += (i+1) as u32 },
            _ => {},
        }
    }

    Some(result)
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

fn compare_lists(left: &List, right: &List) -> ListOrder {

    for i in 0..std::cmp::max(left.elements.len(), right.elements.len()) {
        let left_element = left.elements.get(i);
        let right_element = right.elements.get(i);

        let (left_element, right_element) = match (left_element, right_element) {
            (None, Some(_)) => { return ListOrder::LeftFirst },
            (Some(_), None) => { return ListOrder::RightFirst },
            (Some(left), Some(right)) => (left, right),
            _ => { panic!(); }, // This should not be possible
        };
    
        let list_order = match (left_element, right_element) {
            (ListElement::Number(left_number), ListElement::Number(right_number)) => {
                if left_number < right_number {
                    ListOrder::LeftFirst
                } else if left_number > right_number {
                    ListOrder::RightFirst
                } else {
                    ListOrder::Equal
                }
            },
            (ListElement::Number(left_num), ListElement::List(right_list)) => {
                let left_list = List { elements: vec![ListElement::Number(*left_num)] };
                compare_lists(&left_list, right_list)
            },
            (ListElement::List(left_list), ListElement::Number(right_num)) => {
                let right_list = List { elements: vec![ListElement::Number(*right_num)] };
                compare_lists(left_list, &right_list)
            },
            (ListElement::List(left_list), ListElement::List(right_list)) => {
                compare_lists(left_list, right_list)
            },
        };
        match list_order {
            ListOrder::Equal => {},
            other_order => { return other_order },
        }
    }

    ListOrder::Equal
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
        assert_eq!(part_two(&input), Some(140 as u32));
    }
}
