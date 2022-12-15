use std::cmp::{min, max};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Material {
    Air,
    Rock,
    Sand
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SandAction {
    Fall((usize, usize)),
    AtRest((usize, usize)),
    Abyss,
}

type Data = Vec<Vec<Material>>;

// Each line is a rock structure
// A -> B -> C
// where each arrow is a straight horizontal/vertical line
// from one coord to the next
#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    use Material::*;
    // first pass: read all the lines and put them in a more convenient format
    let mut rocks = vec![];
    // min/max x and y for sizing the grid?
    let mut xmin = 600;
    let mut xmax = 0;
    let mut ymin = 600;
    let mut ymax = 0;
    // Coordinates: x goes right and y goes down
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let mut rock_structure = vec![];
        for coord in line.split(" -> ") {
            let (x,y) = coord.split_once(",").unwrap();
            let x: usize = x.parse()?;
            let y: usize = y.parse()?;
            rock_structure.push((x,y));

            xmin = min(xmin, x);
            xmax = max(xmax, x);
            ymin = min(ymin, y);
            ymax = max(ymax, y);
        }
        rocks.push(rock_structure);
    }

    // arbitrarily margining the grid
    // could probably use x/ymin to make the grid smaller i guess
    // Note that the grid is [y][x]: row then col
    let mut grid = vec![vec![Air; xmax + 2]; ymax + 2];

    for rock_structure in rocks {
        let mut prev_coord = None;
        for (x, y) in rock_structure {
            match prev_coord {
                Some((prev_x, prev_y)) => {
                    if prev_x == x {// vertical line
                        let y_lower = min(prev_y, y);
                        let y_upper = max(prev_y, y);
                        for row in y_lower..=y_upper {
                            grid[row][x] = Rock
                        }
                    }
                    else {// horizontal line
                        let x_lower = min(prev_x, x);
                        let x_upper = max(prev_x, x);
                        for col in x_lower..=x_upper {
                            grid[y][col] = Rock
                        }
                    }
                    prev_coord = Some((x,y));
                },
                None => prev_coord = Some((x,y)),
            }
        }
    }
    dbg!(xmin, xmax, ymin, ymax);
    Ok(grid)
}

fn sand_step(coord: (usize, usize), grid: &Data) -> SandAction {
    let (row, col) = coord;
    // Basic check: there shouldn't be anything already here
    assert!(grid[row][col] == Material::Air);

    // Bounds check: have we fallen into the abyss?
    if row > grid.len() - 2 || col > grid[0].len() - 2 {
        return SandAction::Abyss
    }

    // Can it fall down?
    if Material::Air == grid[row+1][col]{
        return SandAction::Fall((row+1, col))
    }
    // Can it fall one step down and to the left?
    if Material::Air == grid[row+1][col-1]{
        return SandAction::Fall((row+1, col-1))
    }
    // Can it fall one step down and to the right?
    if Material::Air == grid[row+1][col+1]{
        return SandAction::Fall((row+1, col+1))
    }
    
    // Else it cannot move and comes to rest
    SandAction::AtRest(coord)
}

// Part 1: units of sand until sand falls into the abyss
#[aoc(day14, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let mut grid = input.clone();
    let mut num_sand = 0;
    // Sand comes from the point x=500 y=0
    let mut sand_coord = (0, 500);
    loop {
        //dbg!(num_sand, sand_coord);
        match sand_step(sand_coord, &grid) {
            SandAction::Fall(new_coord) => {
                //dbg!("falling", new_coord);
                sand_coord = new_coord
            },
            SandAction::AtRest(final_coord) => {
                // this sand is done
                grid[final_coord.0][final_coord.1] = Material::Sand;
                //dbg!("sand at rest", final_coord);
                // generate more sand
                num_sand += 1;
                sand_coord = (0, 500);
            },
            SandAction::Abyss => {
                //dbg!("fell in the abyss");
                break
            },
        }
    }

    num_sand
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 24);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}