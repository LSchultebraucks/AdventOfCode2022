extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "input.txt";

    let input = read_input(&filename);

    let part1 = part1(input.0, input.1);
    println!("Part 1: {}", part1);


    let input = read_input(&filename);
    let part2 = part2(input.0, input.1);
    println!("Part 2: {}", part2);
}

fn part1(mut stacks: Vec<Vec<char>>, commands: Vec<Command>) -> String {
    for command in commands {
        stacks = execute_command(stacks, &command);
    }
    let result = read_result(stacks);
    return result;
}

fn part2(mut stacks: Vec<Vec<char>>, commands: Vec<Command>) -> String {
    for command in commands {
        stacks = execute_command_2(stacks, &command);
    }
    let result = read_result(stacks);
    return result;
}

fn read_result(stacks: Vec<Vec<char>>) -> String {
    let mut res = String::new();
    for stack in stacks {
        let last = stack.last();
        if !last.is_none() {
            res = res + stack.last().unwrap().to_string().as_str();
        }
    }
    return res;
}

fn read_input(filename: &str) -> (Vec<Vec<char>>, Vec<Command>) {
    let input = File::open(filename).unwrap();
    let buffered = BufReader::new(input);
    let mut stacks = Vec::new();
    for _index in 0..10 {
        stacks.push(Vec::new());
    }
    let mut commands = Vec::new();
    let mut read_stack = true;
    for line in buffered.lines() {
        let unwrapped_line = line.unwrap();
        if read_stack {
            if unwrapped_line.is_empty() {
                read_stack = false;
            } else {
                let line_bytes = unwrapped_line.as_bytes();
                let mut index_line = 1;
                let mut index_stack = 0;
                while index_line < line_bytes.len() {
                    let c = char::from(line_bytes[index_line]);
                    if c.is_alphabetic() {
                        stacks[index_stack].push(c);
                    }
                    index_stack += 1;
                    index_line += 4;
                }
            }
        } else {
            let line_split: Vec<&str> = unwrapped_line.split(" ").collect();
            let command = Command {
                cnt: line_split.get(1).unwrap().parse::<usize>().unwrap(),
                from: line_split.get(3).unwrap().parse::<usize>().unwrap(),
                to: line_split.get(5).unwrap().parse::<usize>().unwrap()
            };
            commands.push(command);
        }
    }
    for index in 0..10 {
        stacks[index].reverse();
    }
    return (stacks, commands);
}

// CrateMover 9000
fn execute_command(mut stacks: Vec<Vec<char>>, command: &Command) -> Vec<Vec<char>> {
    for _ in 0..command.cnt {
        let item = stacks[command.from-1].pop().unwrap();
        stacks[command.to-1].push(item);
    }
    return stacks;
}

// CrateMover 9001
fn execute_command_2(mut stacks: Vec<Vec<char>>, command: &Command) -> Vec<Vec<char>> {
    let len = stacks[command.to-1].len() + command.cnt;
    stacks[command.to-1].resize(len, 'x');
    for index in 0..command.cnt {
        let item = stacks[command.from-1].pop().unwrap();
        stacks[command.to-1][len-1- index] = item;
    }
    return stacks;
}

struct Command {
    cnt: usize,
    from: usize,
    to: usize
}

#[cfg(test)]
mod tests {
    use crate::{execute_command, part1, part2, read_input};

    #[test]
    fn read_input_examples() {
        let actual = read_input("input_example.txt");
        let stacks = actual.0;
        let commands = actual.1;
        assert_eq!(4, commands.len());
        assert_eq!(1, commands.get(0).unwrap().cnt);
        assert_eq!(2, commands.get(0).unwrap().from);
        assert_eq!(1, commands.get(0).unwrap().to);
        assert_eq!(3, commands.get(1).unwrap().cnt);
        assert_eq!(1, commands.get(1).unwrap().from);
        assert_eq!(3, commands.get(1).unwrap().to);
        assert_eq!(2, stacks.get(0).unwrap().len());
        assert_eq!(3, stacks.get(1).unwrap().len());
        assert_eq!(1, stacks.get(2).unwrap().len());
    }

    #[test]
    fn text_execute_command() {
        let input = read_input("input_example.txt");
        let stacks = input.0;
        let commands = input.1;
        let actual = execute_command(stacks.clone(), &commands.get(0).unwrap());
        assert_eq!(3, actual.get(0).unwrap().len());
        assert_eq!(2, actual.get(1).unwrap().len());
        assert_eq!(1, actual.get(2).unwrap().len());
    }

    #[test]
    fn test_part1_input_example() {
        let input = read_input("input_example.txt");
        let expected = "CMZ";
        let actual = part1(input.0, input.1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_input() {
        let input = read_input("input.txt");
        let expected = "RNZLFZSJH";
        let actual = part1(input.0, input.1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_input_example() {
        let input = read_input("input_example.txt");
        let expected = "MCD";
        let actual = part2(input.0, input.1);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2_input() {
        let input = read_input("input.txt");
        let expected = "CNSFCGJSM";
        let actual = part2(input.0, input.1);
        assert_eq!(expected, actual);
    }
}
