use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    // Read grid from the disk
    let grid = read_grid(input)?;

    let rows = grid.len();
    let cols = grid.get(0)?.len();

    // Calculate number of visible trees
    let mut invisible_trees: u32 = 0;

    for row in 1..(rows - 1) {
        for col in 1..(cols - 1) {
            if !is_tree_visible(&grid, row, col)? {
                invisible_trees += 1;
            }
        }
    }

    // Return total trees - visible trees
    Some((rows * cols) as u32 - invisible_trees)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Read grid from the disk
    let grid = read_grid(input)?;

    let rows = grid.len();
    let cols = grid.get(0)?.len();

    let mut current_max_score: u32 = 0;
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            let scenic_score = calculate_scenic_score(&grid, row, col)?;
            current_max_score = max(current_max_score, scenic_score);
        }
    }

    Some(current_max_score)
}

fn calculate_scenic_score(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> Option<u32> {
    let tree_height = *grid.get(row)?.get(col)?;
    let total_rows = grid.len();
    let total_cols = grid.get(0).unwrap().len();

    // west score
    let mut west_score: u32 = 0;
    if col > 0 {
        for check_col in (0..col).rev() {
            if *grid.get(row)?.get(check_col)? < tree_height {
                west_score += 1;
            } else {
                if check_col > 0 {
                    west_score += 1;
                }
                break;
            }
        }
        west_score = max(west_score, 1);
    }

    // east score
    let mut east_score: u32 = 0;
    for check_col in col+1..total_cols {
        if *grid.get(row)?.get(check_col)? < tree_height {
            east_score += 1;
        } else {
            if check_col < total_cols {
                east_score += 1;
            }
            break;
        }
    }
    if east_score == 0 && col < total_cols {
        east_score = 1;
    }

    // north score
    let mut north_score: u32 = 0;
    if row > 0 {
        for check_row in (0..row).rev() {
            if *grid.get(check_row)?.get(col)? < tree_height {
                north_score += 1;
            } else {
                if check_row > 0 {
                    north_score += 1;
                }
                break;
            }
        }
        north_score = max(north_score, 1);
    }

    // south score
    let mut south_score: u32 = 0;
    for check_row in row+1..total_rows {
        if *grid.get(check_row)?.get(col)? < tree_height {
            south_score += 1;
        } else {
            if check_row < total_rows {
                south_score += 1;
            }
            break;
        }
    }
    if south_score == 0 && row < total_rows {
        south_score = 1;
    }
    
    Some(west_score * east_score * north_score * south_score)
}

fn is_tree_visible(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> Option<bool> {
    let tree_height = *grid.get(row)?.get(col)?;
    let total_rows = grid.len();
    let total_cols = grid.get(0).unwrap().len();

    fn tree_visible_from_direction(max_direction_height: Option<u8>, tree_height: u8) -> bool {
        match max_direction_height {
            Some(max_height) => {
                return max_height < tree_height;
            },
            None => {
                return ! tree_height == 0;
            }
        }
    }

    // check from west
    let mut max_direction_height: Option<u8> = None;
    for check_col in 0..col {
        match max_direction_height {
            Some(previous_max_height) => {
                let check_height = *grid.get(row)?.get(check_col)?;
                max_direction_height = Some(max(previous_max_height, check_height));
            },
            None => {
                max_direction_height = Some(*grid.get(row)?.get(check_col)?)
            }
        }
    }
    if tree_visible_from_direction(max_direction_height, tree_height) {
        return Some(true);
    }

    // check from east
    let mut max_direction_height: Option<u8> = None;
    for check_col in col+1..total_cols {
        match max_direction_height {
            Some(previous_max_height) => {
                let check_height = *grid.get(row)?.get(check_col)?;
                max_direction_height = Some(max(previous_max_height, check_height));
            },
            None => {
                max_direction_height = Some(*grid.get(row)?.get(check_col)?)
            }
        }
    }
    if tree_visible_from_direction(max_direction_height, tree_height) {
        return Some(true);
    }

    // check from north
    let mut max_direction_height: Option<u8> = None;
    for check_row in 0..row {
        match max_direction_height {
            Some(previous_max_height) => {
                let check_height = *grid.get(check_row)?.get(col)?;
                max_direction_height = Some(max(previous_max_height, check_height));
            },
            None => {
                max_direction_height = Some(*grid.get(check_row)?.get(col)?)
            }
        }
    }
    if tree_visible_from_direction(max_direction_height, tree_height) {
        return Some(true);
    }

    // check from south
    let mut max_direction_height: Option<u8> = None;
    for check_row in row+1..total_rows {
        match max_direction_height {
            Some(previous_max_height) => {
                let check_height = *grid.get(check_row)?.get(col)?;
                max_direction_height = Some(max(previous_max_height, check_height));
            },
            None => {
                max_direction_height = Some(*grid.get(check_row)?.get(col)?)
            }
        }
    }
    if tree_visible_from_direction(max_direction_height, tree_height) {
        return Some(true);
    }

    Some(false)
}

fn read_grid(input: &str) -> Option<Vec<Vec<u8>>> {
    // Get number of rows
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines.first()?.len();

    let mut grid: Vec<Vec<u8>> = vec![];
    for row in 0..rows {
        let mut row_trees: Vec<u8> = vec![];
        let trees: &str = lines.get(row).unwrap();
        let trees_split: Vec<char> = trees.chars().collect();
        for col in 0..cols {
            row_trees.push(trees_split.get(col)?.to_digit(10).unwrap() as u8);
        }
        grid.push(row_trees);
    }


    Some(grid)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8 as u32));
    }
}
