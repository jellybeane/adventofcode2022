use std::{collections::HashSet, hash::Hash};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = (Vec<u8>, Vec<u8>);

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut result: Vec<Data> = vec![];
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
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
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

#[aoc(day3, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    unimplemented!()
}