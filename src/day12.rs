use std::vec;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Data {
    start: (usize, usize),
    end: (usize, usize),
    grid: Vec<Vec<usize>> // the heightmap
}

// Puzzle input is a grid of letters
// Lowercase a-z indicates elevation (increasing from a to z)
// S is start (elevation a) and E is end (elevation z)
#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut start = (0, 0);
    let mut end= (0, 0);
    let mut grid = vec![];

    for (i, line) in input.lines().enumerate(){
        let mut row = vec![];
        for (j, c) in line.as_bytes().iter().enumerate() {
            let elevation = match c {
                b'S' => {
                    start = (i, j);
                    0
                },
                b'E' => {
                    end = (i, j);
                    25
                },
                b'a'..=b'z' => (c - b'a') as usize,
                _ => unreachable!()
            };
            row.push(elevation);
        }
        grid.push(row)
    }

    Ok(Data{start, end, grid})
}
#[derive(Eq, PartialEq)]
pub struct Node {
    row: usize,
    col: usize,
    distance: usize
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Given a location and elevation grid, 
/// return the locations I can get to accounting for the height rule
fn get_neighbors(location: (usize, usize), grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)>
{
    let mut neighbors = vec![];
    let num_rows = grid.len() as isize;
    let num_cols = grid[0].len() as isize;
    // Up, Down, Right, Left
    let offsets = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    for offset in offsets {
        // needs to be isize in order to check for -1
        let row = (location.0 as isize) + offset.0;
        let col = (location.1 as isize) + offset.1;
        // does the neighbor actually exist, and is elevation change OK?
        if row >= 0 && row < num_rows && col >= 0 && col < num_cols 
                && grid[row as usize][col as usize] <= grid[location.0][location.1] + 1 {
            neighbors.push((row as usize, col as usize));
        }
    }

    neighbors
}

/// Given a start location and elevation grid,
/// return the distance to every location
fn red_blob(start: (usize, usize), grid: &Vec<Vec<usize>>) -> Vec<Vec<usize>>
{
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    // distance from start to each location
    let mut distance = vec![vec![usize::MAX; num_cols]; num_rows];
    distance[start.0][start.1] = 0;

    let start_node = Node {
        row: start.0,
        col: start.1,
        distance: 0
    };
    let mut frontier:Vec<Node> = vec![start_node];

    while let Some(node) = frontier.pop() {
        // TODO get neighbors
        let neighbors = get_neighbors((node.row, node.col), grid);
        for neighbor in neighbors {
            let new_distance = node.distance + 1;
            if new_distance < distance[neighbor.0][neighbor.1] {
                let neighbor_node = Node {
                    row: neighbor.0, 
                    col: neighbor.1, 
                    distance: new_distance
                };
                frontier.push(neighbor_node);
                distance[neighbor.0][neighbor.1] = new_distance;
            }
        }
        frontier.sort();
    }

    distance
}

// Part 1: what is the fewest steps to get from Start to End?
#[aoc(day12, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let distances = red_blob(input.start, &input.grid);

    distances[input.end.0][input.end.1]
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 31);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}