use std::collections::HashSet;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u32> {
    // read moves
    let moves = read_moves(input)?;
    
    // empty list of visited spots
    let mut visited_spots: HashSet<(i32, i32)> = HashSet::new();

    // Positions of the two parts of the snake
    // Both start at (0, 0)
    let mut head: [i32; 2] = [0; 2];
    let mut tail: [i32; 2] = [0; 2];

    // Add start position to the set
    visited_spots.insert((tail[0], tail[1]));

    for head_movement in moves {
        head = movement_after(head, &head_movement);
        // head is now in position

        // is tail within 1 spot of head?
        if (head[0] - tail[0]).abs() <= 1 && (head[1] - tail[1]).abs() <= 1 {
            // we don't need to move
            continue;
        }

        // we need to move, but in which direction?
        // follow the head directly!
        tail = movement_after(tail, &head_movement);

        // Do we need to move again?
        if (head[0] - tail[0]).abs() + (head[1] - tail[1]).abs() > 1 {
            if head_movement == Move::Up || head_movement == Move::Down {
                // we need to move horizontally
                tail[0] = head[0];
            } else {
                // we need to move vertically
                tail[1] = head[1];
            }
        }

        visited_spots.insert((tail[0], tail[1]));
    }

    Some(visited_spots.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
                match line.chars().nth(0)? {
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
        assert_eq!(part_one(&input), Some(13 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1 as u32));
    }
}
