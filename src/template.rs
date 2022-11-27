use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = usize;

#[aoc_generator(dayX)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    unimplemented!()
}

#[aoc(dayX, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    unimplemented!()
}

#[aoc(dayX, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    unimplemented!()
}