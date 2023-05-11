const NUM_ROCKS: usize = 2022;

use std::fmt;

#[derive(Clone, Debug)]
enum Jet {
    Left,
    Right,
}

struct Piece {
    spaces: Vec<[i32; 2]>,
}

impl Piece {
    fn from_number(number: usize) -> Self {
        Self {
            spaces: match number % 5 {
                0 => {
                    // ####
                    vec![
                        [0, 2], [0, 3], [0, 4], [0, 5],
                    ]
                },
                1 => {
                    // .#.
                    // ###
                    // .#.
                    vec![
                                [2, 3],
                        [1, 2], [1, 3], [1, 4],
                                [0, 3],
                    ]
                },
                2 => {
                    // ..#
                    // ..#
                    // ###
                    vec![
                                        [2, 4],
                                        [1, 4],
                        [0, 2], [0, 3], [0, 4],
                    ]
                },
                3 => {
                    // #
                    // #
                    // #
                    // #
                    vec![
                        [0, 5],
                        [0, 4],
                        [0, 3],
                        [0, 2],
                    ]
                },
                4 => {
                    // ##
                    // ##
                    vec![
                        [1, 2], [1, 3],
                        [1, 4], [1, 5],
                    ]
                },
                _ => {
                    // A number modulu 5 cannot be anything except 0-4.
                    // This case can never be reached, but for some reason
                    // the compiler requires it. I thought Rust's type
                    // system was supposed to be smart!
                    vec![]
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Column {
    spaces: Vec<[bool; 7]>,
    highest_rock: u32,
}

impl Column {
    fn new() -> Self {
        Self {
            spaces: vec![],
            highest_rock: 0,
        }
    }

    fn drop_piece(&mut self, piece_num: usize, jets: &Vec<Jet>) {
        let piece = Piece::from_number(piece_num);
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut to_return = String::new();

        // Indicator
        to_return.push_str(format!("Tower of height {}:\n", self.highest_rock).as_str());
        // Top 3 rows (empty)
        to_return.push_str("|.......|\n|.......|\n|.......|\n");
        
        // Content of each row
        for row_index in (0..self.spaces.len()).rev() {
            let row = self.spaces.get(row_index).unwrap();
            for space in row {
                to_return.push_str(match *space {
                    true => { "#" },
                    false => { "." },
                });
            }
            to_return.push_str("\n");
        }

        // Bottom row (always the same)
        to_return.push_str("+-------+\n");

        write!(f, "{}", to_return)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let jets = read_input(input)?;
    Some(calculate_tower_height(&jets))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn calculate_tower_height(jets: &Vec<Jet>) -> u32 {

    let mut column = Column::new();
    println!("Created tower.\n{}\nDropping pieces...", &column);

    for piece_num in 0..NUM_ROCKS {
        column.drop_piece(piece_num, jets);
    }

    0
}

fn read_input(input: &str) -> Option<Vec<Jet>> {
    let mut jets: Vec<Jet> = vec![];

    for c in input.trim().chars() {
        jets.push(match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => {
                println!("Invalid jet direction read: '{}'", c);
                return None;
            }
        });
    }

    Some(jets)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068_u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
