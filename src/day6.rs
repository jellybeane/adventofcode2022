use std::{collections::HashSet, ops::Add};
use std::hash::Hash;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = String;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    // the input is one big line
    Ok(input.to_string())
}

// how many characters need to be inspected
// before we get to a window that's all unique?
// basically https://www.reddit.com/r/adventofcode/comments/zdw0u6/2022_day_6_solutions/iz3nbei/
fn until_all_unique(input: &String, window_size: usize) -> usize {
    input.as_bytes()
    .windows(window_size) // Sliding windows of the given size
    .enumerate() // number each window
    .find(|(_i, window)| { // The first time the predicate is true
        all_unique(*window)
    })
    .unwrap()
    .0 // we only care about the iteration number, not the actual chars
    .add(window_size) // index of the last char checked
}

// are all the elements in this iterator
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
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    until_all_unique(&input, 4)
}

// How many characters need to be processed before we see
// a 14-character window that's all unique?
#[aoc(day6, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    until_all_unique(&input, 14)
}