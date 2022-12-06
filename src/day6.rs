use std::collections::HashSet;
use std::hash::Hash;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = String;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    // the input is one big line
    let vec: Vec<String> = vec![input.to_string()];
    Ok(vec)
}

// basically https://stackoverflow.com/a/46767732
fn all_unique<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut set = HashSet::new();
    iter.into_iter().all(move |x| set.insert(x))
}

// How many characters need to be processed before we see
// a 4-character window that's all unique?
#[aoc(day6, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let line = input[0].as_bytes();
    let mut num = 4;
    for window in line.windows(4) {
        if all_unique(window)
        {
            break;
        }
        num += 1;
    }
    num
}

// How many characters need to be processed before we see
// a 14-character window that's all unique?
#[aoc(day6, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let line = input[0].as_bytes();
    let mut num = 14;
    for window in line.windows(14) {
        if all_unique(window)
        {
            break;
        }
        num += 1;
    }
    num
}