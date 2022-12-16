use std::{collections::HashSet, ops::RangeInclusive};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

pub struct Sensor{
    x: isize,
    y: isize,
    radius: isize, //Manhattan distance to closest beacon
}

#[derive(Hash, PartialEq, Eq)]
pub struct Beacon{
    x: isize,
    y: isize,
}

type Data = (Vec<Sensor>, HashSet<Beacon>);

// it's non-negative but eh
fn manhattan(a: (isize, isize), b:(isize, isize)) -> isize
{
    (a.0-b.0).abs() + (a.1-b.1).abs()
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Result<Data> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Data> {
    let mut sensors = vec![];
    // input has duplicate beacons (closest to more than one sensor)
    let mut beacons = HashSet::new();
    for line in input.lines() {
        // format is
        // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        // +x goes right, +y goes down
        let (sensor_line, beacon_line) = line.split_once(": ").unwrap();
        
        let (_, beacon_line) = beacon_line.split_once("at ").unwrap();
        let (x,y) = beacon_line.split_once(", ").unwrap();
        let x: isize = x[2..].parse()?;
        let y: isize = y[2..].parse()?;
        let beacon = Beacon {x,y};

        let (_, sensor_line) = sensor_line.split_once("at ").unwrap();
        let (x,y) = sensor_line.split_once(", ").unwrap();
        let x: isize = x[2..].parse()?;
        let y: isize = y[2..].parse()?;
        let radius = manhattan((x,y), (beacon.x, beacon.y));
        let sensor = Sensor{x, y, radius};

        sensors.push(sensor);
        beacons.insert(beacon);
    }

    Ok((sensors, beacons))
}

fn add_range(ranges: &mut Vec<RangeInclusive<isize>>, new_range: RangeInclusive<isize>) {
    let mut overlap_indices = vec![];
    // find indices of all ranges that overlap
    // reverse order to make removing them easier
    for i in (0..ranges.len()).into_iter().rev() {
        // if new_range is fully contained, we don't need to do anything
        if ranges[i].contains(&new_range.start())
        && ranges[i].contains(new_range.end()) 
        {
            assert!(overlap_indices.is_empty());
            return
        }

        if ranges[i].end() >= new_range.start() && ranges[i].start() <= new_range.end()
                || ranges[i].contains(new_range.end())
        {
            // overlap:we'll need to merge this range with the new range
            overlap_indices.push(i);
        }
    }

    if overlap_indices.is_empty() {
        // no overlap: just add the new range
        ranges.push(new_range)
    }
    else {
        // remove the existing ranges and add their union
        let mut lower_bounds = vec![];
        let mut upper_bounds = vec![];
        lower_bounds.push(*new_range.start());
        upper_bounds.push(*new_range.end());
        for i in overlap_indices {
            let overlapping_range = ranges.remove(i);
            lower_bounds.push(*overlapping_range.start());
            upper_bounds.push(*overlapping_range.end());
        }
        lower_bounds.sort();
        upper_bounds.sort();
        let &lb = lower_bounds.first().unwrap();
        let &ub = upper_bounds.last().unwrap();
        let unioned = lb..=ub;
        return add_range(ranges, unioned);
    }
}

// Part 1: in a given row, how many positions cannot contain a beacon
#[aoc(day15, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input, 2_000_000)
}
fn solve_part1_inner(input: &Data, y_to_check: isize) -> usize {
    let (sensors, beacons) = input.clone();

    let mut covered_ranges = vec![];

    for sensor in sensors {
        // does this sensor's range overlap the row to check?
        if sensor.y - sensor.radius <= y_to_check
                && sensor.y + sensor.radius >= y_to_check {
            
            let delta_y = (sensor.y - y_to_check).abs();
            let delta_x = sensor.radius - delta_y;
            assert!(delta_x >= 0);
            // go leftwards and rightwards from the center
            let lb = sensor.x - delta_x;
            let ub = sensor.x + delta_x;
            add_range(&mut covered_ranges, lb..=ub);
        }
    }

    let mut num_covered = 0;
    for range in &covered_ranges {
        num_covered += range.end() - range.start() + 1;
    }
    //dbg!(num_covered);
    
    // do not double-count the spots where beacons already are
    let mut num_beacons_on_line = 0;

    for beacon in beacons {
        if beacon.y == y_to_check {
            num_beacons_on_line += 1;
        }
    }
    //dbg!(num_beacons_on_line);
    
    //dbg!(covered_ranges);

    (num_covered - num_beacons_on_line) as usize
}

// Part 2
// There is only one possible location where a secret beacon could be
// with x and y coordinates in the range [0, 4_000_000]
// Where is it?
// "Tuning Frequency": x*4_000_000 + y
#[aoc(day15, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
    // hmm part 1 takes 697.8277ms for me
    // so doing the same thing row by row is not the way to go...

    let max_coord = 20;
    // let max_coord = 4_000_000;

    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1_inner(&input, 10);

        assert_eq!(result, 26);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 56000011);
    }
}