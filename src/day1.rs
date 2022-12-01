use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = Vec<usize>;

// Each line has the Calorie value of an item
// Each Elf's inventory is separated by a blank line
#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut result: Vec<Vec<usize>> = vec![];
    let mut cur_elf: Vec<usize> = vec![];
    for line in input.lines() {
        if line.is_empty() {
            result.push(cur_elf);
            cur_elf = vec![];
        }
        else {
            let value: usize = line.parse()?;
            cur_elf.push(value);
        }
    }
    result.push(cur_elf);

    Ok(result)
}

// Total Calories carried by the elf 
// carrying the most Calories
#[aoc(day1, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let mut biggest_total = 0;
    for elf in input {
        let elf_total: usize = elf.iter().sum();
        if elf_total > biggest_total {
            biggest_total = elf_total;
        }
    }
    biggest_total
}

// Total Calories of the top 3 elves
#[aoc(day1, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let mut elf_totals: Vec<usize> = vec![];
    for elf in input {
        let elf_total: usize = elf.iter().sum();
        elf_totals.push(elf_total);
    }
    
    elf_totals.sort();
    elf_totals.iter().rev().take(3).sum()
}