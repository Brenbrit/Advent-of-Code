const NUM_ROCKS_1: usize = 2022;
const NUM_ROCKS_2: usize = 1000000000000;
//const NUM_ROCKS_2: usize = 1000;
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
    last_move_on_row: Vec<Vec<usize>>,
    highest_rock: u64,
}

impl Column {
    fn new() -> Self {
        Self {
            spaces: vec![],
            last_move_on_row: vec![],
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

            if ! can_move {
                match move_direction {
                    MoveDirection::Down => {
                        // We can't move down
                        // Therefore, this piece has come to rest.
                        // Apply changes to column.
                        let mut max_y_position = 0;
                        for piece_part in &piece.spaces {
                            self.spaces.get_mut(piece_part[1] as usize).unwrap()[(piece_part[0] as usize)] = true;
                            if piece_part[1] > max_y_position {
                                max_y_position = piece_part[1];
                            }
                        }

                        // Update the max height of the column
                        let max_y_position = max_y_position as u64;
                        if max_y_position + 1 > self.highest_rock {

                            // make room in last_move_on_row
                            for _ in self.highest_rock..(max_y_position + 1) {
                                self.last_move_on_row.push(vec![]);
                            }

                            self.highest_rock = max_y_position + 1;
                        }

                        // update the list of which piece num touched each row
                        for piece_part in &piece.spaces {
                            let y_position = piece_part[1] as usize;
                            let mut last_edited_list = self.last_move_on_row[y_position].clone();
                            last_edited_list.push(piece_num);
                            self.last_move_on_row[y_position] = last_edited_list;
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

            to_return.push_str(&format!("| <- row {}", row_index));

            let row_end = match self.last_move_on_row.get(row_index) {
                Some(list) => format!(", last edited list {:?}\n", list),
                None => "\n".to_owned(),
            };
            to_return.push_str(&row_end);
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
    Some(calculate_tower_height_by_chunks(&jets, NUM_ROCKS_2))
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

fn calculate_tower_height_by_chunks(jets: &Vec<Jet>, num_rocks: usize) -> u64 {
    let mut column = Column::new();
    let mut global_jet_index: usize = 0;
    let mut piece_num = 0;
    let mut pattern_period = 1;
    let mut pattern_start_point: usize = 0;

    'period_loop: loop {
        'start_point_loop: for start_point in 0..(pattern_period as usize) {
            // Is our tower tall enough for accurate pattern detection?
            let needed_height = start_point + (3 * pattern_period);
            while column.highest_rock < (needed_height as u64) {
                global_jet_index = column.drop_piece(piece_num, jets, global_jet_index);
                piece_num += 1;
            }

            // Determine if this start point and period forms a pattern
            for period_index in 0..pattern_period {
                // Get indexes of rows to compare
                let first_pattern_row_index = start_point + period_index;
                let second_pattern_row_index = start_point + pattern_period + period_index;
                let third_pattern_row_index = start_point + (pattern_period * 2) + period_index;

                let first_row = column.spaces.get(first_pattern_row_index).unwrap();
                let second_row = column.spaces.get(second_pattern_row_index).unwrap();
                let third_row = column.spaces.get(third_pattern_row_index).unwrap();
                
                if first_row != second_row || first_row != third_row || second_row != third_row {
                    // These rows don't match. Keep searching.
                    continue 'start_point_loop;
                }
            }

            println!("Pattern found!");
            pattern_start_point = start_point;
            break 'period_loop;
        }

        // We did not find a pattern with this period. Try the next one!
        pattern_period += 1;
    }

    // pattern_start_point
    // pattern_period
    // column: list<[bool; 7]>

    println!("Pattern starts at row index {} and has period {} rows.", pattern_start_point, pattern_period);

    // How many pieces were dropped *before* the pattern pieces?
    let edited_list_before_pattern = column.last_move_on_row.get(pattern_start_point - 1).unwrap();
    let pieces_before_pattern = edited_list_before_pattern.iter().max().unwrap();
    let first_pattern_piece = pieces_before_pattern + 1;
    let last_pattern_piece = *column.last_move_on_row.get(pattern_start_point + pattern_period - 1).unwrap().iter().max().unwrap();
    dbg!(first_pattern_piece);

    // The number of pieces in each pattern period
    let pattern_pieces = last_pattern_piece - first_pattern_piece + 1;
    println!("Pattern starts after {} pieces and {} rows.", pieces_before_pattern, pattern_start_point - 1);
    println!("Pattern is {} pieces and {} rows long.", pattern_pieces, pattern_period);
    
    // How many times does the pattern need to be repeated?
    let patterns_needed = (num_rocks - pieces_before_pattern) / pattern_pieces;
    let pieces_needed_after_pattern = (num_rocks - pieces_before_pattern) % pattern_pieces;
    println!("The pattern will need to be repeated {} times, and then {} pieces need to be dropped.", patterns_needed, pieces_needed_after_pattern);

    let mut extra_pieces_height = 0;
    let last_edited_lower_bound = first_pattern_piece - 1 + pieces_needed_after_pattern;
    println!("Searching for the first row where last_edited >= {}", last_edited_lower_bound);
    println!("First pattern piece: {}, pieces needed after pattern: {}, lower bound: {}", first_pattern_piece, pieces_needed_after_pattern, last_edited_lower_bound);
    
    for extra_row in (0..pattern_period).rev() {
        // We are looking for the first row that the desired piece has edited
        // The desired piece's column will contain last_edited_lower_bound

        // Subtract one here because the start of the pattern *is* the first extra piece.
        let extra_row_index = extra_row + pattern_start_point - 1;
        let extra_row_last_edited = column.last_move_on_row.get(extra_row_index).unwrap();
        
        if extra_row_last_edited.contains(&last_edited_lower_bound) {
            extra_pieces_height = extra_row;
            println!("First row to match extra row criteria is {}", extra_row_index);
            break;
        }
    }

    println!("Dropping the extra {} pieces resulted in {} more rows.", pieces_needed_after_pattern, extra_pieces_height);
    println!("Rows before pattern starts: {}", pattern_start_point - 1);
    println!("Patterns needed ({}) * pattern period ({}): {}", patterns_needed, pattern_period, patterns_needed * pattern_period);
    println!("Rows from extra pieces after pattern: {}", extra_pieces_height);
    println!("Finally tally:\n  Before pattern: {}\n  From pattern: {}\n  After pattern: {}", pattern_start_point-1, (patterns_needed * pattern_period), extra_pieces_height);

    //println!("Tower after calculations are performed: \n{}", column);

    (pattern_start_point + (patterns_needed * pattern_period) + extra_pieces_height) as u64
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
        //assert_eq!(part_one(&input), Some(3068_u32));

        // Test part 2 solver with part 1 data
        let jets = read_input(&input).unwrap();
        let chunk_answer = calculate_tower_height_by_chunks(&jets, NUM_ROCKS_1);
        if chunk_answer != 3068_u64 {
            let mut column = Column::new();
            let mut global_jet_index: usize = 0;

            for piece_num in 0..NUM_ROCKS_1 {
                global_jet_index = column.drop_piece(piece_num, &jets, global_jet_index);
            }

            println!("Correct tower: \n{}", column);
        }
        assert_eq!(chunk_answer, 3068_u64);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288_u64));
    }
}
