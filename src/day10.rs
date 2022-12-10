use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

pub enum Instruction {
    Add(isize),
    Noop
}

// Two commands
// addx V: Takes two cycles. The X register is increased to the value V
// noop: Takes one cycle
#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Result<Vec<Instruction>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Instruction>> {
    use Instruction::*;
    let mut commands = vec![];
    for line in input.lines() {
        if line.starts_with("noop") {
            commands.push(Noop)
        }
        else {
            let (_, num) = line.split_once(" ").unwrap();
            let num = num.parse()?;
            commands.push(Add(num));
        }
    }
    Ok(commands)
}

// Part 1: What is the sum of the signal strength during the 
// 20th, 60th, 100th, 140th, 180th, and 220th cycles?
// Signal strength = cycle number * X value
#[aoc(day10, part1)]
pub fn solve_part1(input: &[Instruction]) -> isize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Instruction]) -> isize {
    use Instruction::*;

    let cycles_to_check= vec![20, 60, 100, 140, 180, 220];
    let mut total = 0;
    let mut x:isize = 1;
    let mut cycle = 1;
    for command in input {
        // handle the command
        let cycles_to_add;
        let val_to_add;
        match command {
            Add(val) => {
                cycles_to_add = 2;
                val_to_add = Some(val);
            },
            Noop => {
                cycles_to_add = 1;
                val_to_add = None;
            },
        }

        // increment cycles, checking if it's one we care about
        for _i in 1..=cycles_to_add {
            let signal = cycle * x;
            if cycles_to_check.contains(&cycle) {
                //dbg!(cycle, x, signal);
                total += signal;
            }
            cycle += 1;
        }

        // update X if needed
        if let Some(val) = val_to_add {
            x += val;
        }
    }
    total
}

// X is the center pixel of a 3-wide sprite on a CRT screen 40 wide and 6 high.
// Draw a single pixel per cycle left-to-right 
// until you reach the end of the row, then go one row down.
// Leftmost pixel is 0, rightmost is 39.
// # if current pixel overlaps the sprite, else .

// Render the image given by the input for the answer
#[aoc(day10, part2)]
pub fn solve_part2(input: &[Instruction]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Instruction]) -> usize {
    use Instruction::*;
    let mut x:isize = 1;
    // pixels are 0 indexed while cycles are 1 indexed
    let mut cur_pix = 0;
    let mut cycle = 1;
    let mut cur_row:String = String::from("");
    for command in input {
        // handle the command
        let cycles_to_add;
        let val_to_add;
        match command {
            Add(val) => {
                cycles_to_add = 2;
                val_to_add = Some(val);
            },
            Noop => {
                cycles_to_add = 1;
                val_to_add = None;
            },
        }

        // increment cycles, rendering pixel
        for _i in 1..=cycles_to_add {
            let pix = if cur_pix == x-1 || cur_pix == x || cur_pix == x+1 {
                '#'
            }
            else {
                '.'
            };
            cur_row.push(pix);

            cycle += 1;
            cur_pix += 1;

            // have we completed a row?
            if cycle % 40 == 1 {
                println!("{}", cur_row);
                cur_row = String::from("");
                cur_pix = 0;
            }
        }

        // update X if needed
        if let Some(val) = val_to_add {
            x += val;
        }
    }
    // the last row
    println!("{}", cur_row);

    // the solution is in the printlines: return doesn't matter
    0
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 13140);
    }

}

