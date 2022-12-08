use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

// 2D array
type Data = Vec<usize>;

// Input is a rectangular grid of digits 0-9
#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Result<Vec<Data>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Data>> {
    let mut array = vec![];
    for line in input.lines() {
        let mut row = vec![];
        // parse each individual digit as a number
        for &c in line.as_bytes() {
            let number: usize = c.to_string().parse()?;
            row.push(number);
        }
        array.push(row);
    }
    Ok(array)
}

// A tree is "visible" if there are no other trees
// of the same height or taller between it and the edge
#[aoc(day8, part1)]
pub fn solve_part1(input: &[Data]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Data]) -> usize {
    let mut visible = 0;
    let num_rows = input.len();
    let num_cols = input[0].len();
    dbg!(num_rows, num_cols);
    // there's definitely a better way...
    for (i, row) in input.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            let mut blocked = false;
            // to the left
            for col in (0..j).rev() {
                if input[i][col] >= height {
                    //dbg!(i, j, "blocked on left");
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                visible += 1;
                continue;
            }

            // to the right
            blocked = false;
            for col in j+1..num_cols {
                if input[i][col] >= height {
                    //dbg!(i, j, "blocked on right");
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                visible += 1;
                continue;
            }

            // up
            blocked = false;
            for row in (0..i).rev() {
                if input[row][j] >= height {
                    //dbg!(i, j, "blocked on top");
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                visible += 1;
                continue;
            }

            // down
            blocked = false;
            for row in i+1..num_rows {
                if input[row][j] >= height {
                    //dbg!(i, j, "blocked on bottom");
                    blocked = true;
                    break;
                }
            }
            if !blocked {
                visible += 1;
            }
        }
    }

    visible
}

// A tree's "scenic score" is the product of the viewing distance
// in each of the four directions
#[aoc(day8, part2)]
pub fn solve_part2(input: &[Data]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Data]) -> usize {
    let mut high_score = 0;
    let num_rows = input.len();
    let num_cols = input[0].len();
    dbg!(num_rows, num_cols);
    // there's definitely a better way...
    for (i, row) in input.iter().enumerate() {
        for (j, &height) in row.iter().enumerate() {
            
            // to the left
            let mut leftscore = j;
            for col in (0..j).rev() {
                if input[i][col] >= height {
                    //dbg!(i, j, "blocked on left");
                    leftscore = j - col;
                    break;
                }
            }

            // to the right
            let mut rightscore = num_cols - 1 - j;
            for col in j+1..num_cols {
                if input[i][col] >= height {
                    //dbg!(i, j, "blocked on right");
                    rightscore = col - j;
                    break;
                }
            }

            // up
            let mut upscore = i;
            for row in (0..i).rev() {
                if input[row][j] >= height {
                    //dbg!(i, j, "blocked on top");
                    upscore = i - row;
                    break;
                }
            }

            // down
            let mut downscore = num_rows - 1 - i;
            for row in i+1..num_rows {
                if input[row][j] >= height {
                    //dbg!(i, j, "blocked on bottom");
                    downscore = row - i;
                    break;
                }
            }

            let score = leftscore * rightscore * upscore * downscore;
            //dbg!(i, j, score);
            if high_score < score {
                high_score = score
            }
        }
    }

    high_score
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"30373
25512
65332
33549
35390
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 8);
    }
}