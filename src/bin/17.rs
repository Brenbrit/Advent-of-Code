const NUM_ROCKS_1: usize = 2022;
const NUM_ROCKS_2: usize = 1000000000000;
// 0 means that rocks start at the same y index as the max rock.
// Set to 3 to start rocks 3 spaces above the highest rock.
const DROP_HEIGHT: u8 = 3;

use std::fmt;

#[derive(Clone, Debug)]
enum Jet {
    Left,
    Right,
}

enum NextMove {
    Jet,
    Down,
}

enum MoveDirection {
    Right,
    Left,
    Down,
}

struct Piece {
    spaces: Vec<[i64; 2]>,
}

impl Piece {
    fn from_number(number: usize) -> (Self, u8) {
        let (piece_shape, height) = match number % 5 {
            0 => {
                // ####
                (vec![
                    [2, 0], [3, 0], [4, 0], [5, 0],
                ], 1)
            },
            1 => {
                // .#.
                // ###
                // .#.
                (vec![
                            [3, 2],
                    [2, 1], [3, 1], [4, 1],
                            [3, 0],
                ], 3)
            },
            2 => {
                // ..#
                // ..#
                // ###
                (vec![
                                    [4, 2],
                                    [4, 1],
                    [2, 0], [3, 0], [4, 0],
                ], 3)
            },
            3 => {
                // #
                // #
                // #
                // #
                (vec![
                    [2, 3],
                    [2, 2],
                    [2, 1],
                    [2, 0],
                ], 4)
            },
            4 => {
                // ##
                // ##
                (vec![
                    [2, 1], [3, 1],
                    [2, 0], [3, 0],
                ], 2)
            },
            _ => {
                // A number modulu 5 cannot be anything except 0-4.
                // This case can never be reached, but for some reason
                // the compiler requires it. I thought Rust's type
                // system was supposed to be smart!
                (vec![], 0)
            }
        };

        (Self { spaces: piece_shape }, height)
    }
}

#[derive(Clone, Debug)]
struct Column {
    spaces: Vec<[bool; 7]>,
    highest_rock: u64,
}

impl Column {
    fn new() -> Self {
        Self {
            spaces: vec![],
            highest_rock: 0,
        }
    }

    fn drop_piece(&mut self, piece_num: usize, jets: &Vec<Jet>, mut global_jet_index: usize) -> usize {
        let (mut piece, piece_height) = Piece::from_number(piece_num);

        // Make sure the board is big enough to hold our piece
        self.allocate_space_for_piece(piece_height);

        // Update piece's y positions to proper height
        let drop_start_height = (self.highest_rock + u64::from(DROP_HEIGHT)) as i64;
        for piece_part_index in 0..piece.spaces.len() {
            piece.spaces.get_mut(piece_part_index).unwrap()[1] += drop_start_height;
        }

        let mut next_move = NextMove::Jet;

        loop {
            let move_direction = match next_move {
                NextMove::Jet => {
                    let jet_direction = match jets.get(global_jet_index % (jets.len())).unwrap() {
                        Jet::Left => MoveDirection::Left,
                        Jet::Right => MoveDirection::Right,
                    };
                    global_jet_index += 1;

                    jet_direction
                },
                NextMove::Down => MoveDirection::Down,
            };

            let can_move = piece_can_move(&piece, &move_direction, &self.spaces);

            /*match move_direction {
                MoveDirection::Right => {
                    println!("Piece index {} wants to move right. It can do this => {}", piece_num, can_move);
                },
                MoveDirection::Left => {
                    println!("Piece index {} wants to move left. It can do this => {}", piece_num, can_move);
                },
                MoveDirection::Down => {
                    println!("Piece index {} wants to move down. It can do this => {}", piece_num, can_move);
                },
            }

            draw_falling_piece(&self, &piece);*/

            if ! can_move {
                match move_direction {
                    MoveDirection::Down => {
                        // We can't move down
                        // Therefore, this piece has come to rest.
                        // Apply changes to column.
                        let mut max_y_position = 0;
                        for piece_part in piece.spaces {
                            self.spaces.get_mut(piece_part[1] as usize).unwrap()[(piece_part[0] as usize)] = true;
                            if piece_part[1] > max_y_position {
                                max_y_position = piece_part[1];
                            }
                        }

                        // Update the max height of the column
                        let max_y_position = max_y_position as u64;
                        if max_y_position + 1 > self.highest_rock {
                            self.highest_rock = max_y_position + 1;
                        }

                        break;
                    }
                    _ => {
                        // We can't move to the side.
                        // This is ok - we'll just move down the next time around.
                        next_move = NextMove::Down;
                        continue;
                    },
                }
            }

            // Looks like we *can* move!
            // Let's do that.
            let (move_x, move_y) = match move_direction {
                MoveDirection::Right => (1, 0),
                MoveDirection::Left => (-1, 0),
                MoveDirection::Down => (0, -1),
            };
            let mut new_piece_spaces: Vec<[i64; 2]> = vec![];
            for piece_part_index in 0..piece.spaces.len() {
                let old_part_position = piece.spaces.get(piece_part_index).unwrap();
                let new_part_position = [old_part_position[0] + move_x, old_part_position[1] + move_y];
                new_piece_spaces.push(new_part_position);
            }
            piece.spaces = new_piece_spaces;

            // Set the next move
            next_move = match next_move {
                NextMove::Down => NextMove::Jet,
                NextMove::Jet => NextMove::Down,
            }
        }

        // return jet_index for future use
        global_jet_index
    }

    fn allocate_space_for_piece(&mut self, piece_height: u8) {
        // empty spaces above highest rock
        let empty_rows = (self.spaces.len() as u64) - self.highest_rock;

        // Number of spaces we need
        let needed_spaces = DROP_HEIGHT + piece_height;

        // Do we need to increase the size of the board?
        if needed_spaces > (empty_rows as u8) {
            for _ in 0..(needed_spaces - (empty_rows as u8)) {
                self.spaces.push([false; 7]);
            }
        }
    }
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut to_return = String::new();

        // Indicator
        to_return.push_str(format!("Tower of height {}:\n", self.highest_rock).as_str());
        // Top 3 rows (empty)
        // to_return.push_str("|.......|\n|.......|\n|.......|\n");
        
        // Content of each row
        for row_index in (0..self.spaces.len()).rev() {
            to_return.push('|');
            let row = self.spaces.get(row_index).unwrap();
            for space in row {
                to_return.push_str(match *space {
                    true => { "#" },
                    false => { "." },
                });
            }
            to_return.push_str("|\n");
        }

        // Bottom row (always the same)
        to_return.push_str("+-------+\n");

        write!(f, "{}", to_return)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let jets = read_input(input)?;
    Some(calculate_tower_height(&jets, NUM_ROCKS_1) as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let jets = read_input(input)?;
    Some(calculate_tower_height(&jets, NUM_ROCKS_2))
}

fn piece_can_move(piece: &Piece, move_direction: &MoveDirection, column: &Vec<[bool; 7]>) -> bool {

    // The offsets that signify where this piece wants to move
    // E.x. (-1, 0) means piece is moving left, (0, -1) means piece is moving down.
    let (x_offset, y_offset) = match move_direction {
        MoveDirection::Right => (1, 0),
        MoveDirection::Left => (-1, 0),
        MoveDirection::Down => (0, -1),
    };

    for piece_part in &piece.spaces {
        // Determine part's next spot
        let part_next_spot = [piece_part[0] + x_offset, piece_part[1] + y_offset];

        // This part is going to be in the position of an existing part,
        // which we can guarantee is valid.
        if piece.spaces.contains(&part_next_spot) { continue; }

        // Is this part's next position out of bounds?
        if part_next_spot[0] < 0 || part_next_spot[0] > 6 || part_next_spot[1] < 0 {
            return false;
        }

        // Is this part's next position conflicting with an existing settled part?
        if column.get((part_next_spot[1]) as usize).unwrap()[(part_next_spot[0]) as usize] == true {
            return false;
        }
    }

    true
}

fn calculate_tower_height(jets: &Vec<Jet>, num_rocks: usize) -> u64 {

    let mut column = Column::new();
    let mut global_jet_index: usize = 0;

    for piece_num in 0..num_rocks {
        global_jet_index = column.drop_piece(piece_num, jets, global_jet_index);
    }

    column.highest_rock
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
        assert_eq!(part_two(&input), Some(1514285714288_u64));
    }
}
