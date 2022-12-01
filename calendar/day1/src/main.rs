use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "input.txt";

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut input = Vec::new();

    for line in buffered.lines() {
        input.push(line.unwrap());
    }

    let mut elf_calories: Vec<i32> = Vec::new();
    let mut current_elf_calories = 0;
    for line in input {
        if line.is_empty() {
            elf_calories.push(current_elf_calories);
            current_elf_calories = 0;
        } else {
            current_elf_calories += line.parse::<i32>().unwrap();
        }
    }

    elf_calories.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", elf_calories[0]);

    let part2 = elf_calories[0] + elf_calories[1]+ elf_calories[2];

    println!("Part 2: {}", part2);

    Ok(())
}
