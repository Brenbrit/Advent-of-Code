pub fn part_one(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.lines().collect();

    let plays = &split.into_iter().map(line_to_plays_1).collect();

    Some(calculate_score(plays))
}

#[derive(Clone, Debug)]
enum Play {
    Rock,
    Paper,
    Scissors
}

fn calculate_score(rounds: &Vec<(Play, Play)>) -> u32 {
    let mut score = 0;

    for (my_play, their_play) in rounds {

        // Add rps score
        let rps_score = match my_play {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };

        // Add win/lose score
        let win_score = match my_play {
            Play::Rock => {
                match their_play {
                    Play::Rock => 3,
                    Play::Paper => 0,
                    Play::Scissors => 6
                }
            },
            Play::Paper => {
                match their_play {
                    Play::Rock => 6,
                    Play::Paper => 3,
                    Play::Scissors => 0,
                }
            },
            Play::Scissors => {
                match their_play {
                    Play::Rock => 0,
                    Play:: Paper => 6,
                    Play::Scissors => 3,
                }
            },
        };

        score += rps_score + win_score;

    }

    score
}

fn line_to_plays_1(line: &str) -> (Play, Play) {
    let their_play = match line.chars().next().unwrap() {
        'A' => Play::Rock,
        'B' => Play::Paper,
        _ => Play::Scissors,
    };
    let my_play = match line.chars().nth(2).unwrap() {
        'X' => Play::Rock,
        'Y' => Play::Paper,
        _ => Play::Scissors,
    };

    (my_play, their_play)
}

fn line_to_plays_2(line: &str) -> (Play, Play) {
    let their_play = match line.chars().next().unwrap() {
        'A' => Play::Rock,
        'B' => Play::Paper,
        _ => Play::Scissors,
    };
    let my_play = match line.chars().nth(2).unwrap() {
        'X' => match their_play {
            // we need to lose
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        },
        // Y: we need to draw
        'Y' => their_play.clone(),
        _ => match their_play {
            // we need to win
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        },
    };

    (my_play, their_play)
}


pub fn part_two(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.lines().collect();

    let plays = &split.into_iter().map(line_to_plays_2).collect();

    Some(calculate_score(plays))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15_u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12_u32));
    }
}
