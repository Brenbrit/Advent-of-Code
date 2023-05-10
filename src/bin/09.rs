use std::{collections::HashSet, cmp::{max, min}};

const STARATING_SPACE: [i32; 2] = [0, 0];
const PT2_NUM_KNOTS: usize = 10;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u32> {
    tail_visited_spots(2, input)
}

pub fn part_two(input: &str) -> Option<u32> {
    tail_visited_spots(PT2_NUM_KNOTS, input)
}

fn tail_visited_spots(num_knots: usize, input: &str) -> Option<u32> {
    // read moves
    let moves = read_moves(input)?;
    
    // empty list of visited spots
    let mut tail_visited_spots: HashSet<(i32, i32)> = HashSet::new();

    let mut knots: Vec<[i32; 2]> = vec![];
    for _ in 0..num_knots {
        knots.push(STARATING_SPACE);
    }
    //_print_rope(&knots);

    for head_movement in moves {
        let old_knots = knots.clone();
        knots.clear();

        let head = movement_after(*old_knots.first()?, &head_movement);
        knots.push(head);
        // head is now in position

        for following_knot_index in 1..num_knots {
            let leader = knots.last()?;
            let follower = old_knots.get(following_knot_index)?;
            knots.push(follow_knot(leader, *follower));
        }

        let tail = knots.last().unwrap();
        tail_visited_spots.insert((tail[0], tail[1]));

        //_print_rope(&knots);
    }

    Some(tail_visited_spots.len() as u32)
}

fn follow_knot(leading_knot: &[i32; 2], following_knot: [i32; 2]) -> [i32; 2] {
    // is tail within 1 spot of head?
    if (leading_knot[0] - following_knot[0]).abs() <= 1 
    && (leading_knot[1] - following_knot[1]).abs() <= 1 {
        // we don't need to move
        return following_knot;
    }

    let mut to_return = following_knot;

    // we need to move, but in which direction?
    if (leading_knot[0] - following_knot[0]).abs() > 1 {
        // we must move in the x direction
        // move diagonally if necessary
        if leading_knot[1] != following_knot[1] {
            if leading_knot[1] > to_return[1] {
                to_return[1] += 1;
            } else {
                to_return[1] -= 1;
            }   
        }

        if leading_knot[0] > to_return[0] {
            to_return[0] += 1;
        } else {
            to_return[0] -= 1;
        }
    } else {
        // we must move in the y direction
        // move diagonally if necessary
        if leading_knot[0] != following_knot[0] {
            if leading_knot[0] > to_return[0] {
                to_return[0] += 1;
            } else {
                to_return[0] -= 1;
            }
        }

        if leading_knot[1] > to_return[1] {
            to_return[1] += 1;
        } else {
            to_return[1] -= 1;
        }    
    }

    to_return
}

fn _print_rope(rope: &Vec<[i32; 2]>) {
    let mut min_values: [i32; 2] = [i32::MAX, i32::MAX];
    let mut max_values: [i32; 2] = [i32::MIN, i32::MIN];
    for knot_position in rope {
        max_values[0] = max(max_values[0], knot_position[0]);
        max_values[1] = max(max_values[1], knot_position[1]);
        min_values[0] = min(min_values[0], knot_position[0]);
        min_values[1] = min(min_values[1], knot_position[1]);
    }

    for row in (min_values[0]..=max_values[0]).rev() {
        for col in (min_values[1]..=max_values[1]).rev() {
            if row == 0 && col == 0 {
                print!("+");
            } else if rope.contains(&[row, col]) {
                let index = rope.iter().position(|&r| r == [row, col]).unwrap();
                if index == 0 {
                    print!("H")
                } else {
                    print!("{}", index)
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn movement_after(starting_position: [i32; 2], input_move: &Move) -> [i32; 2] {
    match input_move {
        Move::Up => {
            [starting_position[0], starting_position[1] + 1]
        },
        Move::Down => {
            [starting_position[0], starting_position[1] - 1]
        },
        Move::Left => {
            [starting_position[0] - 1, starting_position[1]]
        },
        Move::Right => {
            [starting_position[0] + 1, starting_position[1]]
        },
    }
}

fn read_moves(input: &str) -> Option<Vec<Move>> {
    let mut moves: Vec<Move> = vec![];
    let lines: Vec<&str> = input.lines().collect();

    for line in lines {
        let num_moves = line.split(' ').nth(1)?.parse::<u32>().unwrap();
        for _ in 0..num_moves {
            moves.push(
                match line.chars().next()? {
                    'U' => {Move::Up},
                    'D' => {Move::Down},
                    'L' => {Move::Left},
                    'R' => {Move::Right},
                    _ => {
                        println!("Read an invalid character.");
                        return None;
                    }
                }
            );
        }
    }

    Some(moves)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13_u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1_u32));
    }
}
