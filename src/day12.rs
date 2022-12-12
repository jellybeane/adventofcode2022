use std::{vec, collections::VecDeque};

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

/// Given a location, elevation grid, and elevation change rule,
/// return the locations I can get to
fn get_neighbors<F>(location: (usize, usize), grid: &Vec<Vec<usize>>,
        is_valid_move: &F) 
        -> Vec<(usize, usize)>
        where F: Fn(&Vec<Vec<usize>>, (usize, usize), (usize, usize))  -> bool
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
                && is_valid_move(grid, (row as usize, col as usize), location) {
            neighbors.push((row as usize, col as usize));
        }
    }

    neighbors
}

/// Given a start location, elevation grid, and elevation change rule
/// return the distance to every location
fn red_blob<F>(start: (usize, usize), grid: &Vec<Vec<usize>>, is_valid_move: F) 
        -> Vec<Vec<usize>>
        where F: Fn(&Vec<Vec<usize>>, (usize, usize), (usize, usize))  -> bool
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
    let mut frontier = VecDeque::new();
    frontier.push_back(start_node);

    // Breadth-First Search
    // It'd be Depth-First if this were a Stack instead of a Queue
    while let Some(node) = frontier.pop_front() {
        let neighbors = get_neighbors((node.row, node.col), grid, &is_valid_move);
        for neighbor in neighbors {
            let new_distance = node.distance + 1;
            if new_distance < distance[neighbor.0][neighbor.1] {
                let neighbor_node = Node {
                    row: neighbor.0, 
                    col: neighbor.1, 
                    distance: new_distance
                };
                frontier.push_back(neighbor_node);
                distance[neighbor.0][neighbor.1] = new_distance;
            }
        }
    }

    distance
}

// Part 1: what is the fewest steps to get from Start to End?
#[aoc(day12, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    // pass in is_valid_move as a closure: given the elevation grid, neighbor coords, and location coord,
    // is going from location to neighbor allowed?
    let distances = red_blob(input.start, &input.grid, 
        |grid, (row, col), location|{
            grid[row][col] <= grid[location.0][location.1] + 1
        }
    );

    distances[input.end.0][input.end.1]
}

// Part 2: the fewest steps to move from any square with elevation a
// to the end
#[aoc(day12, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    // pass in is_valid_move as a closure: given the elevation grid, neighbor coords, and location coord,
    // is going from neighbor to location allowed?
    let distances = red_blob(input.end, &input.grid, 
        |grid, (row, col), location|{
            grid[location.0][location.1] <= grid[row][col] + 1 
        }
    );

    let num_rows = input.grid.len();
    let num_cols = input.grid[0].len();
    let mut fewest_steps = usize::MAX;
    for i in 0..num_rows {
        for j in 0..num_cols {
            if input.grid[i][j] == 0 && distances[i][j] < fewest_steps {
                fewest_steps = distances[i][j]
            }
        }
    }
    fewest_steps
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

        assert_eq!(result, 29);
    }
}