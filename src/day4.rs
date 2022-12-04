use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

type Data = ((usize, usize),(usize, usize));

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut result: Vec<Data> = vec![];
    for line in input.lines() {
        // is there a nice way to map over these or something?
        let (left, right) = line.split_once(',').unwrap();
        let (left0, left1) = left.split_once('-').unwrap();
        let (right0, right1) = right.split_once('-').unwrap();
        //dbg!(left0, left1, right0, right1);
        let left: (usize, usize) = (left0.parse()?, left1.parse()?);
        let right: (usize, usize) = (right0.parse()?, right1.parse()?);
        result.push((left, right));
    }
    Ok(result)
}

// In how many assignment pairs does one range fully contain the other?
#[aoc(day4, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let mut counter = 0;
    for pair in input {
        let ((left0, left1), (right0, right1)) = pair;
        if (left0 <= right0 && left1 >= right1) || (right0 <= left0 && right1 >= left1) {
            counter += 1;
        }
    }
    counter
}

// In how many assignment pairs do the ranges overlap?
#[aoc(day4, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let mut counter = 0;
    for pair in input {
        let ((left0, left1), (right0, right1)) = pair;
        if left0 <= right1 && right0 <= left1 {
            counter += 1;
        }
    }
    counter
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 4);
    }
}