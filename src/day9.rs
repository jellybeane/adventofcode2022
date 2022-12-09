use std::{collections::HashSet, vec};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Down,
    Up
}

type Data = (Direction, usize);

// Input is a series of Moves, indicating a Direction and a Number
// R(ight), L(eft), D(own), U(p)
#[aoc_generator(day9 )]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    use Direction::*;
    let mut moves = vec![];
    for line in input.lines()
    {
        let (left, right) = line.split_once(" ").unwrap();
        let direction = match left {
            "R" => Right,
            "L" => Left,
            "D" => Down,
            "U" => Up,
            _ => Right // error????
        };
        moves.push((direction, right.parse()?))
    }

    Ok(moves)
}

// A rope with Head and Tail
// At the end of each move, the head and tail must be touching
// (diagonal and same-space count)

/// Given a step direction, update the position of the head
fn update_head(direction: &Direction, head: (i32, i32)) -> (i32, i32)
{
    let mut new_head = head;
    match direction {
        Direction::Right => new_head.0 += 1,
        Direction::Left => new_head.0 -= 1,
        Direction::Down => new_head.1 -= 1,
        Direction::Up => new_head.1 += 1,
    }
    new_head
}

/// Given the updated head position, update the tail
fn update_tail_1(new_head: (i32, i32), tail: (i32, i32)) -> (i32, i32)
{
    let mut new_tail = tail;

    // update tail
    // If the head is two steps directly up/down/left/right,
    // then the tail moves one step in that direction
    if new_head.0 == new_tail.0 { 
        // up
        if new_head.1 == new_tail.1 + 2 {
            new_tail.1 += 1;
        }
        // down
        else if new_head.1 == new_tail.1 - 2{
            new_tail.1 -= 1;
        }
    }
    else if new_head.1 == new_tail.1 { 
        // right
        if new_head.0 == new_tail.0 + 2 {
            new_tail.0 += 1;
        }
        // left
        else if new_head.0 == new_tail.0 - 2{
            new_tail.0 -= 1;
        }
    }
    // Else if the head and tail aren't touching and aren't in the same row/col,
    // the tail moves one step diagonally
    else if i32::abs(new_head.0 - tail.0) + i32::abs(new_head.1 - new_tail.1) > 2 {
        new_tail.0 += if new_head.0 > new_tail.0 {1} else {-1};
        new_tail.1 += if new_head.1 > new_tail.1 {1} else {-1};
    }
    
    new_tail
}

#[aoc(day9 , part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    // Head and tail start at the same position overlapping
    let mut head = (0, 0); // x, y
    let mut tail = (0, 0);

    // set of all positions the tail has visited
    let mut tail_visited = HashSet::new();
    for (direction, steps) in input {
        for _i in 0..*steps {
            head = update_head(direction, head);
            tail = update_tail_1(head, tail);
            
            tail_visited.insert(tail);
        }
    }

    tail_visited.len()
}

// Now there are 10 knots instead of just Head and Tail
#[aoc(day9 , part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let num_knots = 10;
    let mut knots = vec![(0, 0); num_knots];

    let mut tail_visited = HashSet::new();
    for (direction, steps) in input {
        for _i in 0..*steps {
            knots[0] = update_head(direction, knots[0]);
            for i in 1..num_knots {
                knots[i] = update_tail_1(knots[i-1], knots[i]);
            }
            tail_visited.insert(knots[num_knots-1]);
        }
    }
    tail_visited.len()

}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 1);
    }
}