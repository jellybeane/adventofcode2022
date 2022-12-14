use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut stacks = vec![];
    // doing stacks manually because parsing is hard
    // bottom of the stack comes first
    stacks.push(vec!['L','N','W','T','D']);
    stacks.push(vec!['C','P','H']);
    stacks.push(vec!['W','P','H','N','D','G','M','J']);
    stacks.push(vec!['C','W','S','N','T','Q','L']);
    stacks.push(vec!['P','H','C','N']);
    stacks.push(vec!['T','H','N','D','M','W','Q','B']);
    stacks.push(vec!['M','B','R','J','G','S','L']);
    stacks.push(vec!['Z','N','W','G','V','B','R','T']);
    stacks.push(vec!['W','G','D','N','P','L']);

    // only do the move commands
    // format is "move A from B to C"
    let mut commands = vec![];
    for line in input.lines() {
        if !line.starts_with("move") {
            continue;
        }
        let words: Vec<&str> = line.split(" ").collect();
        // i'm sorry Q__Q
        let a: usize = words[1].parse()?;
        let b: usize = words[3].parse()?;
        let c: usize = words[5].parse()?;
        // given stack numbers are 1 indexed but I want 0 indexed
        commands.push((a,b-1,c-1));
    }
    Ok((stacks, commands))
}

// Part 1: crates move one by one
// popping off the stack to be pushed onto the other
#[aoc(day5, part1)]
pub fn solve_part1(input: &Data) -> String {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> String {
    let (stacks, commands) = input;
    // ??? ownership ??? can't declare 'mut stacks' above
    let mut stacks = stacks.clone();
    // move all the crates
    for &(num_move, from_stack, to_stack) in commands {
        for _i in 0..num_move {
            // the word "crate" is reserved
            let c = stacks[from_stack].pop().unwrap();
            stacks[to_stack].push(c);
        }
    }
    // what crate is at the top of each stack?
    let mut top = String::new();
    for stack in stacks {
        top.push(*stack.last().unwrap());
    }
    top
}

// Part 2: multiple crates can be moved at once
#[aoc(day5, part2)]
pub fn solve_part2(input: &Data) -> String {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> String {
    let (stacks, commands) = input;
    // ??? ownership ??? can't declare 'mut stacks' above
    let mut stacks = stacks.clone();
    // move all the crates
    let mut to_move = vec![];
    for &(num_move, from_stack, to_stack) in commands {
        for _i in (0..num_move).rev() {
            // the word "crate" is reserved
            let c = stacks[from_stack].pop().unwrap();
            to_move.push(c);
        }
        // append is "draining": to_move will be empty after this
        stacks[to_stack].append(&mut to_move);
    }
    // what crate is at the top of each stack?
    let mut top = String::new();
    for stack in stacks {
        top.push(*stack.last().unwrap());
    }
    top
}