use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;


#[derive(Debug, PartialEq, Eq)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Shape::Rock => match other {
                Shape::Rock => Some(Ordering::Equal),
                Shape::Paper => Some(Ordering::Less),
                Shape::Scissors => Some(Ordering::Greater),
                _ => None
            },
            Shape::Paper => match other {
                Shape::Rock => Some(Ordering::Greater),
                Shape::Paper => Some(Ordering::Equal),
                Shape::Scissors => Some(Ordering::Less),
                _ => None
            },
            Shape::Scissors => match other {
                Shape::Rock => Some(Ordering::Less),
                Shape::Paper => Some(Ordering::Greater),
                Shape::Scissors => Some(Ordering::Equal),
                _ => None
            },
            _ => None,
        }
    }
}

type Data = (Shape, Shape);

// Rock Paper Scissors
// "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors"
// Second column: X Rock, Y Paper, Z Scissors
#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut result: Vec<Data> = vec![];
    for line in input.lines() {
        let split: Vec<&str> = line.split(" ").collect();
        let left = match split[0] {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            _ => Shape::Rock, // wait how do I error?
        };
        let right = match split[1] {
            "X" => Shape::Rock,
            "Y" => Shape::Paper,
            "Z" => Shape::Scissors,
            _ => Shape::Rock, // how do I error?
        };
        result.push((left,right));
    }

    Ok(result)
}

// Scoring:
// Shape you selected: 1 Rock, 2 paper, 3 Scissors
// Outcome: 0 loss, 3 draw, 6 win
// What would your total score be if everything goes exactly according to your strategy guide?
#[aoc(day2, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let mut score: usize = 0;
    for round in input {
        let (opponent, me) = round;
        let shapescore = match me {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
            _ => 0, // ???
        };
        let outcomescore = match me.partial_cmp(opponent) {
            Some(compare) => match compare {
                Ordering::Less => 0,
                Ordering::Equal => 3,
                Ordering::Greater => 6,
            },
            None => 0 // ???

        };
        score += shapescore + outcomescore;
    }
    score
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    unimplemented!()
}