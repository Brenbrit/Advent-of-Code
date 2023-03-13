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

/*
--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

 */

pub fn part_one(input: &str) -> Option<u32> {
    let split: Vec<&str> = input.lines().collect();

    let plays = &split.into_iter().map(|line| line_to_plays_1(line)).collect();

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
    let their_play = match line.chars().nth(0).unwrap() {
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
    let their_play = match line.chars().nth(0).unwrap() {
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

    let plays = &split.into_iter().map(|line| line_to_plays_2(line)).collect();

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
        assert_eq!(part_one(&input), Some(15 as u32));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12 as u32));
    }
}
