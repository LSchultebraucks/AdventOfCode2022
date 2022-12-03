extern crate core;

use std::fs::File;
use std::io::{BufRead, BufReader, Error};

fn main() -> Result<(), Error> {
    let filename = "input.txt";

    let input = read_input(&filename);

    let part1 = part1(&input);
    println!("Part 1: {}", part1);

    let part2 = part2(&input);
    println!("Part 2: {}", part2);

    Ok(())
}

fn read_input(filename: &str) -> Vec<String> {
    let input = File::open(filename).unwrap();
    let buffered = BufReader::new(input);

    let mut input = Vec::new();

    for line in buffered.lines() {
        input.push(line.unwrap());
    }
    return input;
}

fn part1(input: &Vec<String>) -> i32 {
    let mut result = 0;
    for rucksack in input {
        let compartments = split_rucksack(rucksack);
        let c = common_char(compartments.0, compartments.1);
        result += priority_of(c);
    }
    return result;
}

fn part2(input: &Vec<String>) -> i32 {
    let mut result = 0;
    let mut index = 2;
    while index < input.len() {
        let c = common_char_group(input.get(index).unwrap(), input.get(index - 1).unwrap(), input.get(index - 2).unwrap());
        result += priority_of(c);
        index += 3;
    }
    return result;
}

fn common_char(a: &str, b: &str) -> char {
    for candidate in a.chars() {
        if b.contains(candidate) {
            return candidate;
        }
    }
    panic!("No common char in strings found!");
}

fn common_char_group(a: &str, b: &str, c: &str) -> char {
    for candidate in a.chars() {
        if b.contains(candidate) && c.contains(candidate) {
            return candidate;
        }
    }
    panic!("No common char in strings found!");
}

fn split_rucksack(rucksack: &str) -> (&str, &str) {
    return rucksack.split_at(rucksack.len() / 2);
}

fn priority_of(c: char) -> i32 {
    return if c.is_lowercase() {
        (c as i32) - 96
    } else {
        (c as i32) - 38
    } as i32
}



#[cfg(test)]
mod tests {
    use crate::{common_char, common_char_group, part1, part2, priority_of, read_input, split_rucksack};

    #[test]
    fn read_input_examples() {
        let filename = "input_example.txt";
        let actual = read_input(&filename);
        assert_eq!(6, actual.len());
        assert_eq!("vJrwpWtwJgWrhcsFMMfFFhFp", actual.get(0).unwrap());
        assert_eq!("CrZsJsPPZsGzwwsLwLmpwMDw", actual.get(5).unwrap());
    }

     #[test]
     fn part1_input_example() {
         let input_example = read_input("input_example.txt");
         let actual = part1(&input_example);
         assert_eq!(157, actual);
     }

    #[test]
    fn part1_input() {
        let input_example = read_input("input.txt");
        let actual = part1(&input_example);
        assert_eq!(7917, actual);
    }

    #[test]
    fn common_char_example() {
        let a = "vJrwpWtwJgWr";
        let b = "hcsFMMfFFhFp";
        let expected = 'p';
        let actual = common_char(a, b);
        assert_eq!(expected, actual)
    }

    #[test]
    fn split_rucksack_example() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let expected_first_compartment = "vJrwpWtwJgWr";
        let expected_second_compartment = "hcsFMMfFFhFp";
        let actual = split_rucksack(rucksack);
        assert_eq!(expected_first_compartment, actual.0);
        assert_eq!(expected_second_compartment, actual.1);
    }

    #[test]
    fn priority_of_examples() {
        let lower_a = 'a';
        let lower_z = 'z';
        let upper_a = 'A';
        let upper_z = 'Z';
        let expected_priority_lower_a = 1;
        let expected_priority_lower_z = 26;
        let expected_priority_upper_a = 27;
        let expected_priority_upper_z = 52;
        assert_eq!(expected_priority_lower_a, priority_of(lower_a));
        assert_eq!(expected_priority_lower_z, priority_of(lower_z));
        assert_eq!(expected_priority_upper_a, priority_of(upper_a));
        assert_eq!(expected_priority_upper_z, priority_of(upper_z));
    }

    #[test]
    fn part2_input_example() {
        let input_example = read_input("input_example.txt");
        let actual = part2(&input_example);
        assert_eq!(70, actual);
    }

    #[test]
    fn common_char_group_example() {
        let a = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let b = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let c = "PmmdzqPrVvPwwTWBwg";
        let expected = 'r';
        let actual = common_char_group(a, b, c);
        assert_eq!(expected, actual)
    }

    #[test]
    fn common_char_group_example_2() {
        let a = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let b = "ttgJtRGJQctTZtZT";
        let c = "CrZsJsPPZsGzwwsLwLmpwMDw";
        let expected = 'Z';
        let actual = common_char_group(a, b, c);
        assert_eq!(expected, actual)
    }
}
