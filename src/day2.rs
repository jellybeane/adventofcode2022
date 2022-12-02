use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

// I feel like I should've implemented some cyclic thing instead :|
impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Shape::*;
        use Ordering::*;
        let result = match (self, other) {
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => Less,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Equal,
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Greater,
        };
        Some(result)
    }
}


// Rock Paper Scissors
// "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors"
// Second column: X, Y, Z
type Data = (char, char);
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut result: Vec<Data> = vec![];
    
    for line in input.lines() {
        if line.is_empty() { continue; }
        let (left, right) = line.split_once(' ')
            .ok_or(anyhow!("Failed to split"))?;
        // getting individual chars is not idiomatic Rust and that's why it's awkward
        // go back to Python if you want that
        // https://stackoverflow.com/questions/33405672/how-can-i-convert-a-one-element-string-into-a-char
        let left = left.chars().next().expect("failed to split");
        let right = right.chars().next().expect("failed to split");
        result.push((left,right));
    }

    Ok(result)
}

// Part 1 Second column: X Rock, Y Paper, Z Scissors
fn read_strat_part_1(lines: &[(char, char)]) -> Vec<(Shape, Shape)> {
    let mut result = vec![];
    for (left, right) in lines {
        let them = match left {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => Shape::Rock, // wait how do I error?
        };
        let me = match right {
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => Shape::Rock, // ???
        };
        result.push((them,me));
    }
    result
}

// Part 2 Second column: how the round needs to end
// X lose, Y draw, Z win
fn read_strat_part_2(lines: &[(char, char)]) -> Vec<(Shape, Ordering)> {
    let mut result = vec![];
    for (left, right) in lines {
        let them = match left {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => Shape::Rock, // wait how do I error?
        };
        let me = match right {
            'X' => Ordering::Less,
            'Y' => Ordering::Equal,
            'Z' => Ordering::Greater,
            _ => Ordering::Less, // hmm
        };
        result.push((them,me));
    }
    result
}

// I want this to be like the opposite of PartialOrd
fn my_play(opponent: Shape, outcome: Ordering) -> Shape {
    // loses to -1, ties to 0, wins to +1, mod 3
    // I was thinking about Java compare when I wrote this
    // even though I literally just wrote PartialOrd .__.
    let shapevec = vec![Shape::Rock, Shape::Paper, Shape::Scissors];
    let their_index: i32 = match opponent {
        Shape::Rock => 0,
        Shape::Paper => 1,
        Shape::Scissors => 2,
    };
    let mut my_index: i32 = match outcome {
        Ordering::Less => their_index - 1,
        Ordering::Equal => their_index,
        Ordering::Greater => their_index + 1,
    };
    // % is negative for negative numbers in rust
    // dbg!(-1%3, 0%3, 1%3, 2%3, 3%3, 4%3);
    if my_index < 0 { my_index += 3; }
    my_index %= 3;
    
    //dbg!((their_index, my_index, outcome));
    shapevec[my_index as usize]
}

// Scoring:
// Shape you selected: 1 Rock, 2 paper, 3 Scissors
// Outcome: 0 loss, 3 draw, 6 win
pub fn scoring(my_shape: Shape, outcome: Ordering) -> usize
{
    let shapescore = match my_shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };
    let outcomescore = match outcome {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    };
    shapescore + outcomescore
}

// What would your total score be if everything goes exactly according to your strategy guide?
#[aoc(day2, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let strat = read_strat_part_1(input);
    let mut score: usize = 0;
    for round in strat {
        let (opponent, me) = round;
        score += scoring(me, me.partial_cmp(&opponent).expect("Failed to compare"));
    }
    score
}

// choose shape so the round ends as indicated
#[aoc(day2, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let strat = read_strat_part_2(input);
    let mut score: usize = 0;
    for round in strat {
        let (opponent, outcome) = round;
        let me = my_play(opponent, outcome);
        //dbg!((opponent, me, outcome));
        score += scoring(me, outcome);
    }
    score
}
