use std::fs::File;
use std::io::{BufReader, BufRead, Error};


const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;
const LOOSE_SCORE: i32 = 0;
const DRAW_SCORE: i32 = 3;
const VICTORY_SCORE: i32 = 6;

fn main() -> Result<(), Error> {
    let filename = "input.txt";

    let input = read_input(&filename);

    let part1 = part1(&input);
    println!("Part 1: {}", part1);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);

    Ok(())
}

fn read_input(filename: &str) -> Vec<(char, char)> {
    let input = File::open(filename).unwrap();
    let buffered = BufReader::new(input);

    let mut input = Vec::new();

    for line in buffered.lines() {
        let line_bytes = line.unwrap().as_bytes().to_owned();
        let opponent = char::from(line_bytes[0]);
        let player = char::from(line_bytes[2]);
        input.push((opponent, player));
    }
    return input;
}

fn part1(input: &Vec<(char, char)>) -> i32 {
    let mut score = 0;
    for game in input {
        score += calc_game_score(game.0, game.1);
    }
    return score;
}

fn part2(input: &Vec<(char, char)>) -> i32 {
    let mut score = 0;
    for game in input {
        score += calc_game_score_part_2(game.0, game.1);
    }
    return score;
}

fn calc_game_score(opponent: char, player: char) -> i32 {
    let mut score = 0;
    if opponent == 'A' {
        if player == 'X' {
            score += ROCK_SCORE;
            score += DRAW_SCORE;
        } else if player == 'Y' {
            score += PAPER_SCORE;
            score += VICTORY_SCORE;
        } else if player == 'Z' {
            score += SCISSORS_SCORE;
            score += LOOSE_SCORE;
        }
    }
    else if opponent == 'B' {
        if player == 'X' {
            score += ROCK_SCORE;
            score += LOOSE_SCORE;
        } else if player == 'Y' {
            score += PAPER_SCORE;
            score += DRAW_SCORE;
        } else if player == 'Z' {
            score += SCISSORS_SCORE;
            score += VICTORY_SCORE;
        }

    }
    else if opponent == 'C' {
        if player == 'X' {
            score += ROCK_SCORE;
            score += VICTORY_SCORE;
        } else if player == 'Y' {
            score += PAPER_SCORE;
            score += LOOSE_SCORE;
        } else if player == 'Z' {
            score += SCISSORS_SCORE;
            score += DRAW_SCORE;
        }
    }
    return score;
}

fn calc_game_score_part_2(opponent: char, outcome: char) -> i32 {
    let mut score = 0;
    if opponent == 'A' {
        if outcome == 'X' {
            score += SCISSORS_SCORE;
            score += LOOSE_SCORE;
        } else if outcome == 'Y' {
            score += ROCK_SCORE;
            score += DRAW_SCORE;
        } else if outcome == 'Z' {
            score += PAPER_SCORE;
            score += VICTORY_SCORE;
        }
    }
    else if opponent == 'B' {
        if outcome == 'X' {
            score += ROCK_SCORE;
            score += LOOSE_SCORE;
        } else if outcome == 'Y' {
            score += PAPER_SCORE;
            score += DRAW_SCORE;
        } else if outcome == 'Z' {
            score += SCISSORS_SCORE;
            score += VICTORY_SCORE;
        }

    }
    else if opponent == 'C' {
        if outcome == 'X' {
            score += PAPER_SCORE;
            score += LOOSE_SCORE;
        } else if outcome == 'Y' {
            score += SCISSORS_SCORE;
            score += DRAW_SCORE;
        } else if outcome == 'Z' {
            score += ROCK_SCORE;
            score += VICTORY_SCORE;
        }
    }
    return score;
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, read_input};

    #[test]
    fn read_input_example() {
        let filename = "input_example.txt";
        let actual = read_input(&filename);
        assert_eq!(actual.len(), 3);
        assert_eq!(actual.get(0).unwrap().0, 'A');
        assert_eq!(actual.get(0).unwrap().1, 'Y');
        assert_eq!(actual.get(1).unwrap().0, 'B');
        assert_eq!(actual.get(1).unwrap().1, 'X');
        assert_eq!(actual.get(2).unwrap().0, 'C');
        assert_eq!(actual.get(2).unwrap().1, 'Z');
    }

    #[test]
    fn part1_input_example() {
        let mut input = Vec::new();
        input.push(('A','Y'));
        input.push(('B','X'));
        input.push(('C','Z'));

        let actual = part1(&input);

        assert_eq!(actual, 15);
    }

    #[test]
    fn part2_input_example() {
        let mut input = Vec::new();
        input.push(('A','Y'));
        input.push(('B','X'));
        input.push(('C','Z'));

        let actual = part2(&input);

        assert_eq!(actual, 12);
    }
}
