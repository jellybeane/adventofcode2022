use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

pub struct Sensor{
    x: isize,
    y: isize,
    radius: isize, //Manhattan distance to closest beacon
}

pub struct Beacon{
    x: isize,
    y: isize,
}

type Data = (Vec<Sensor>, Vec<Beacon>);

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
    let mut beacons = vec![];
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
        beacons.push(beacon);
    }

    Ok((sensors, beacons))
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Data) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &Data) -> usize {
    let (sensors, beacons) = input.clone();

    // how do i use different values for test and real?
    let y_to_check = 10; 
    // let y_to_check = 2_000_000;
    // HashSet of the columns covered by sensor radii
    // it's fine is a point is covered by more than one sensor
    let mut covered = HashSet::new();

    for sensor in sensors {
        // does this sensor's range overlap the row to check?
        if sensor.y - sensor.radius <= y_to_check
                && sensor.y + sensor.radius >= y_to_check {
            
            let delta_y = (sensor.y - y_to_check).abs();
            let delta_x = sensor.radius - delta_y;
            assert!(delta_x >= 0);
            // go leftwards and rightwards from the center
            for i in 0..=delta_x {
                covered.insert(sensor.x - i);
                covered.insert(sensor.x + i);
            }
            // do sensors count?
            if sensor.y == y_to_check {
                covered.remove(&sensor.x);
            }
        }
    }
    
    // do not double-count the spots where beacons already are
    for beacon in beacons {
        if beacon.y == y_to_check {
            covered.remove(&beacon.x);
        }
    }
    covered.len()
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &Data) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &Data) -> usize {
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
        let result = super::solve_part1(&input);

        assert_eq!(result, 26);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 0);
    }
}