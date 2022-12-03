use std::{collections::HashSet};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

// Each line represents a rucksack, divided into two equal components
type Data1 = (Vec<u8>, Vec<u8>);
#[aoc_generator(day3, part1)]
pub fn input_generator1(input: &str) -> Result<Vec<Data1>> {
    input_generator_inner1(input)
}
fn input_generator_inner1(input: &str) -> Result<Vec<Data1>> {
    let mut result: Vec<Data1> = vec![];
    for line in input.lines() {
        let rucksack = line.as_bytes();
        let n = rucksack.len();
        let left = rucksack[0..n/2].to_vec();
        let right = rucksack[n/2..n].to_vec();
        result.push((left, right));
    }
    Ok(result)
}

// Takes the u8 representation of an ASCII char
//Lowercase item types a through z have priorities 1 through 26
//Uppercase item types A through Z have priorities 27 through 52
fn to_priority(item: u8) -> u8 {
    // Lowercase ASCII starts at 97
    if item > 96 {
        return item - 96
    }
    // uppercase ASCII starts at 65
    item - 65 + 27
}

// Find the item type that appears in both compartments of each rucksack.
// What is the sum of the priorities of those item types?
#[aoc(day3, part1)]
pub fn solve_part1(input: &[Data1]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data1]) -> usize {
    //dbg!(input);
    let mut total: usize = 0;
    for (left, right) in input {
        // HashSet conveniently produces Intersections
        let left: HashSet<u8> = left.iter().copied().collect();
        let right: HashSet<u8> = right.iter().copied().collect();
        // we are told there is only one item in both sides
        let intersection = left.intersection(&right).next().unwrap();
        total += to_priority(*intersection) as usize;
    }
    total
}

// Each line still represents a rucksack,
// but they no longer need to be divided
type Data2 = Vec<u8>;
#[aoc_generator(day3, part2)]
pub fn input_generator2(input: &str) -> Result<Vec<Data2>> {
    input_generator_inner2(input)
}
fn input_generator_inner2(input: &str) -> Result<Vec<Data2>> {
    let result: Vec<Data2> = input.lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();
    Ok(result)
}

// Every set of three lines is one group of Elves
// There is one intersection between them
#[aoc(day3, part2)]
pub fn solve_part2(input: &[Data2]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data2]) -> usize {
    let mut total: usize = 0;
    // chunks and chunks_exact conveniently do exactly what is needed
    for elves in input.chunks_exact(3)
    {
        let elf0: HashSet<u8> = elves[0].iter().copied().collect();
        let elf1: HashSet<u8> = elves[1].iter().copied().collect();
        let elf2: HashSet<u8> = elves[2].iter().copied().collect();

        let intersection0: HashSet<u8> = elf0.intersection(&elf1).copied().collect();
        // we are told there is only 1 intersection between the 3 elves
        let badge = intersection0.intersection(&elf2).next().unwrap();
        total += to_priority(*badge) as usize;
    }
    total
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator1(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 157);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator2(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 70);
    }
}