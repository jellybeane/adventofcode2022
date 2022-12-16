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

/// Add a new range to existing ranges
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

/// For a given row, get the positions covered by sensors
fn get_sensor_coverage(sensors: &Vec<Sensor>, y: isize) -> Vec<RangeInclusive<isize>>
{
    let mut covered_ranges = vec![];

    for sensor in sensors {
        // does this sensor's range overlap the row to check?
        if sensor.y - sensor.radius <= y
                && sensor.y + sensor.radius >= y {
            
            let delta_y = (sensor.y - y).abs();
            let delta_x = sensor.radius - delta_y;
            assert!(delta_x >= 0);
            // go leftwards and rightwards from the center
            let lb = sensor.x - delta_x;
            let ub = sensor.x + delta_x;
            add_range(&mut covered_ranges, lb..=ub);
        }
    }

    covered_ranges
}

// Part 1: in a given row, how many positions cannot contain a beacon
#[aoc(day15, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input, 2_000_000)
}
fn solve_part1_inner(input: &Data, y_to_check: isize) -> usize {
    let (sensors, beacons) = input.clone();

    let covered_ranges = get_sensor_coverage(sensors, y_to_check);

    let mut num_covered = 0;
    for range in &covered_ranges {
        num_covered += range.end() - range.start() + 1;
    }
    
    // do not double-count the spots where beacons already are
    for beacon in beacons {
        if beacon.y == y_to_check {
            num_covered -= 1;
        }
    }

    num_covered as usize
}

// Part 2
// There is only one possible location where a secret beacon could be
// with x and y coordinates in the range [0, 4_000_000]
// Where is it?
// "Tuning Frequency": x*4_000_000 + y

fn tuning(x: isize, y: isize) -> usize {
    (x*4_000_000 + y) as usize
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input, 4_000_000)
}
fn solve_part2_inner(input: &Data, max_coord: isize) -> usize {
    let (sensors, _) = input.clone();

    for y in 0..=max_coord {
        let covered_ranges = get_sensor_coverage(sensors, y);

        // does the first sensor start after 0?
        if covered_ranges.first().unwrap().start() > &0 {
            return tuning(covered_ranges.first().unwrap().start()-1, y);
        }
        // does the last sensor end before the max coord?
        if covered_ranges.last().unwrap().end() < &max_coord {
            return tuning(covered_ranges.last().unwrap().end() + 1, y);
        }

        // look at the gaps between ranges
        let mut prev: Option<RangeInclusive<isize>> = None; 
        for range in covered_ranges {
            if let Some(prev_range) = prev {
                // if there's a gap between the end of prev and the start of this
                if range.start() - prev_range.end() > 1 {
                    let x = prev_range.end() + 1;
                    return tuning(x,y);
                }
            }

            prev = Some(range);
        }
    }
    
    dbg!("Gap not found?");
    unreachable!()
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
        let result = super::solve_part2_inner(&input, 20);

        assert_eq!(result, 56000011);
    }
}