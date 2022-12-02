use std::fs::File;
use std::io::{BufReader, BufRead, Error};


const ROCK_SCORE: i32 = 1;
const PAPER_SCORE: i32 = 2;
const SCISSORS_SCORE: i32 = 3;
const LOOSE_SCORE: i32 = 0;
const DRAW_SCORE: i32 = 3;
const VICTORY_SCORE: i32 = 6;
const A: char = 'A';
const B: char = 'B';
const C: char = 'C';
const X: char = 'X';
const Y: char = 'Y';
const Z: char = 'Z';

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
    let score = match opponent {
        A => match player {
            X => ROCK_SCORE + DRAW_SCORE,
            Y => PAPER_SCORE + VICTORY_SCORE,
            Z => SCISSORS_SCORE + LOOSE_SCORE,
            _ => panic!(""),
        },
        B => match player {
            X => ROCK_SCORE + LOOSE_SCORE,
            Y => PAPER_SCORE + DRAW_SCORE,
            Z => SCISSORS_SCORE + VICTORY_SCORE,
            _ => panic!(""),
        }
        C => match player {
            X => ROCK_SCORE + VICTORY_SCORE,
            Y => PAPER_SCORE + LOOSE_SCORE,
            Z => SCISSORS_SCORE + DRAW_SCORE,
            _ => panic!(""),
        },
        _ => panic!(),
    };
    return score;
}

fn calc_game_score_part_2(opponent: char, outcome: char) -> i32 {
    let score = match opponent {
        A => match outcome {
            X => SCISSORS_SCORE + LOOSE_SCORE,
            Y => ROCK_SCORE + DRAW_SCORE,
            Z => PAPER_SCORE + VICTORY_SCORE,
            _ => panic!(""),
        },
        B => match outcome {
            X => ROCK_SCORE + LOOSE_SCORE,
            Y => PAPER_SCORE + DRAW_SCORE,
            Z => SCISSORS_SCORE + VICTORY_SCORE,
            _ => panic!(""),
        }
        C => match outcome {
            X => PAPER_SCORE + LOOSE_SCORE,
            Y => SCISSORS_SCORE + DRAW_SCORE,
            Z => ROCK_SCORE + VICTORY_SCORE,
            _ => panic!(""),
        },
        _ => panic!(),
    };
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
    fn part1_input() {
        let input = read_input("input.txt");
        let actual = part1(&input);
        assert_eq!(actual, 11666);
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

    #[test]
    fn part2_input() {
        let input = read_input("input.txt");
        let actual = part2(&input);
        assert_eq!(actual, 12767);
    }
}
