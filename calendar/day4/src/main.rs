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
fn part1(pairs: &Vec<Pair>) -> i32 {
    let mut result = 0;
    for pair in pairs {
        if is_fully_overlapping(pair) {
            result += 1;
        }
    }
    return result;
}

fn part2(pairs: &Vec<Pair>) -> i32 {
    let mut result = 0;
    for pair in pairs {
        if is_overlapping(pair) {
            result += 1;
        }
    }
    return result;
}


fn read_input(filename: &str) -> Vec<Pair> {
    let input = File::open(filename).unwrap();
    let buffered = BufReader::new(input);
    let mut result = Vec::new();
    for line in buffered.lines() {
        let unrwapped_line = line.unwrap();
        let mut pairs = unrwapped_line.split(",");
        let mut first = pairs.next().unwrap().split("-");
        let mut second = pairs.next().unwrap().split("-");
        let first_section = Section {
            start: first.next().unwrap().parse().unwrap(),
            end: first.next().unwrap().parse().unwrap(),
        };
        let second_section = Section {
            start: second.next().unwrap().parse().unwrap(),
            end: second.next().unwrap().parse().unwrap(),
        };
        let pair = Pair {
            first: first_section,
            second: second_section
        };
        result.push(pair);
    }
    return result;
}

fn is_fully_overlapping(pair: &Pair) ->  bool {
    if pair.first.start <= pair.second.start && pair.first.end >= pair.second.end
        || pair.first.start >= pair.second.start && pair.first.end <= pair.second.end {
            return true;
    };
    return false;
}

fn is_overlapping(pair: &Pair) -> bool {
    if pair.first.start <= pair.second.start && pair.first.end >= pair.second.start
        || pair.first.end <= pair.second.start && pair.first.end >= pair.second.end
        || pair.second.start <= pair.first.start && pair.second.end >= pair.first.start
        || pair.second.end <= pair.first.start && pair.second.end >= pair.first.end {
        return true;
    }
    return false;
}

struct Section {
    start: i32,
    end: i32,
}

struct Pair {
    first: Section,
    second: Section
}

#[cfg(test)]
mod tests {
    use crate::{is_fully_overlapping, is_overlapping, Pair, part1, part2, read_input, Section};

    #[test]
    fn read_input_examples() {
        let actual = read_input("input_example.txt");
        assert_eq!(6, actual.len());
        assert_eq!(2, actual.get(0).unwrap().first.start);
        assert_eq!(4, actual.get(0).unwrap().first.end);
        assert_eq!(6, actual.get(0).unwrap().second.start);
        assert_eq!(8, actual.get(0).unwrap().second.end);
    }

    #[test]
     fn part1_example_input() {
         let input_example = read_input("input_example.txt");
         let actual = part1(&input_example);
         assert_eq!(2, actual);
    }

    #[test]
    fn part1_input() {
        let input_example = read_input("input.txt");
        let actual = part1(&input_example);
        assert_eq!(413, actual);
    }

    #[test]
    fn test_is_fully_overlapping() {
        let not_fully_overlapping_pair = Pair {
            first: Section {
                start: 2,
                end: 4
            },
            second: Section{
                start: 6,
                end: 8
            }
        };
        assert_eq!(false, is_fully_overlapping(&not_fully_overlapping_pair));
        let fully_overlapping_pair = Pair {
            first: Section {
                start: 2,
                end: 8
            },
            second: Section{
                start: 3,
                end: 7
            }
        };
        assert_eq!(true, is_fully_overlapping(&fully_overlapping_pair));
        let another_fully_overlapping_pair = Pair {
            first: Section {
                start: 6,
                end: 6
            },
            second: Section{
                start: 4,
                end: 6
            }
        };
        assert_eq!(true, is_fully_overlapping(&another_fully_overlapping_pair));
    }

    #[test]
    fn test_is_overlapping_pair() {
        let not_overlapping_pair = Pair {
            first: Section {
                start: 2,
                end: 4
            },
            second: Section{
                start: 6,
                end: 8
            }
        };

        assert_eq!(false, is_overlapping(&not_overlapping_pair));
        let overlapping_pair = Pair {
            first: Section {
                start: 5,
                end: 7
            },
            second: Section{
                start: 7,
                end: 9
            }
        };
        assert_eq!(true, is_overlapping(&overlapping_pair));
        let overlapping_pair_2 = Pair {
            first: Section {
                start: 2,
                end: 8
            },
            second: Section{
                start: 3,
                end: 7
            }
        };
        assert_eq!(true, is_overlapping(&overlapping_pair_2));
        let overlapping_pair_3 = Pair {
            first: Section {
                start: 6,
                end: 6
            },
            second: Section{
                start: 4,
                end: 6
            }
        };
        assert_eq!(true, is_overlapping(&overlapping_pair_3));
        let overlapping_pair_4 = Pair {
            first: Section {
                start: 2,
                end: 6
            },
            second: Section{
                start: 4,
                end: 8
            }
        };
        assert_eq!(true, is_overlapping(&overlapping_pair_4));
    }

    #[test]
    fn part2_example_input() {
        let input_example = read_input("input_example.txt");
        let actual = part2(&input_example);
        assert_eq!(4, actual);
    }

    #[test]
    fn part2_input() {
        let input_example = read_input("input.txt");
        let actual = part2(&input_example);
        assert_eq!(806, actual);
    }

}
