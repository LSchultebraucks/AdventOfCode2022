use std::fs::File;
use std::io::{BufReader, BufRead, Error};
use std::str::from_utf8;

fn main() -> Result<(), Error> {
    let path = "input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut input = Vec::new();

    for line in buffered.lines() {
        let line_bytes = line.unwrap().as_bytes().to_owned();
        let opponent = char::from(line_bytes[0]);
        let player = char::from(line_bytes[2]);
        input.push((opponent, player));
    }

    let mut part1 = 0;
    let mut part2 = 0;

    for game in input {
        part1 += calc_game_score(game.0, game.1);
        part2 += calc_game_score_part_2(game.0, game.1);
    }

    println!("Part 1: {}", part1);

    println!("Part 2: {}", part2);

    Ok(())
}

fn calc_game_score(opponent: char, player: char) -> i32 {
    let mut score = 0;
    let rock_score = 1;
    let paper_score = 2;
    let scissors_score = 3;
    let loose_score = 0;
    let draw_score = 3;
    let victory_score = 6;
    if opponent == 'A' {
        if player == 'X' {
            score += rock_score;
            score += draw_score;
        } else if player == 'Y' {
            score += paper_score;
            score += victory_score;
        } else if player == 'Z' {
            score += scissors_score;
            score += loose_score;
        }
    }
    else if opponent == 'B' {
        if player == 'X' {
            score += rock_score;
            score += loose_score;
        } else if player == 'Y' {
            score += paper_score;
            score += draw_score;
        } else if player == 'Z' {
            score += scissors_score;
            score += victory_score;
        }

    }
    else if opponent == 'C' {
        if player == 'X' {
            score += rock_score;
            score += victory_score;
        } else if player == 'Y' {
            score += paper_score;
            score += loose_score;
        } else if player == 'Z' {
            score += scissors_score;
            score += draw_score;
        }
    }
    return score;
}

fn calc_game_score_part_2(opponent: char, outcome: char) -> i32 {
    let mut score = 0;
    let rock_score = 1;
    let paper_score = 2;
    let scissors_score = 3;
    let loose_score = 0;
    let draw_score = 3;
    let victory_score = 6;
    if opponent == 'A' {
        if outcome == 'X' {
            score += scissors_score;
            score += loose_score;
        } else if outcome == 'Y' {
            score += rock_score;
            score += draw_score;
        } else if outcome == 'Z' {
            score += paper_score;
            score += victory_score;
        }
    }
    else if opponent == 'B' {
        if outcome == 'X' {
            score += rock_score;
            score += loose_score;
        } else if outcome == 'Y' {
            score += paper_score;
            score += draw_score;
        } else if outcome == 'Z' {
            score += scissors_score;
            score += victory_score;
        }

    }
    else if opponent == 'C' {
        if outcome == 'X' {
            score += paper_score;
            score += loose_score;
        } else if outcome == 'Y' {
            score += scissors_score;
            score += draw_score;
        } else if outcome == 'Z' {
            score += rock_score;
            score += victory_score;
        }
    }
    return score;
}
