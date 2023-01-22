/*
--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.

Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.

The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your strategy guide?
 */

pub fn part_one(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.lines().collect();

    let mut scores: Vec<u32> = Vec::new();
    scores.push(calculate_score(&split, Play::Rock, Play::Paper, Play::Scissors));
    scores.push(calculate_score(&split, Play::Scissors, Play::Rock, Play::Paper));
    scores.push(calculate_score(&split, Play::Paper, Play::Scissors, Play::Rock));

    scores.sort();
    scores.reverse();

    Some(scores[0])
}

#[derive(Clone)]
enum Play {
    Rock,
    Paper,
    Scissors
}

fn calculate_score(rounds: &Vec<&str>, x_val: Play, y_val: Play, z_val: Play) -> u32 {
    let mut score = 0;

    for round in rounds {
        let (my_play, their_play) = line_to_plays(
            round, 
            x_val.clone(), 
            y_val.clone(), 
            z_val.clone()
        );

        // Add rps score
        score += match my_play {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };

        // Add win/lose score
        score += match my_play {
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
    }

    score
}

fn line_to_plays(line: &str, x_val: Play, y_val: Play, z_val: Play) -> (Play, Play) {
    let my_play = match line.chars().nth(0).unwrap() {
        'A' => Play::Rock,
        'B' => Play::Paper,
        _ => Play::Scissors,
    };
    let their_play = match line.chars().nth(2).unwrap() {
        'X' => x_val,
        'Y' => y_val,
        _ => z_val,
    };

    (my_play, their_play)
}


pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_one(&input), Some(15 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
