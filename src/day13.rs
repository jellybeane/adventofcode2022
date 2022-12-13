use std::{cmp::Ordering, vec};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Data {
    Number(usize),
    List(Vec<Data>)
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Data::Number(me), Data::Number(them)) => me.cmp(them),
            (Data::Number(me), Data::List(_)) => {
                let d = Data::List(vec![Data::Number(*me)]);
                d.cmp(other)
            },
            (Data::List(_), Data::Number(them)) => {
                let d = Data::List(vec![Data::Number(*them)]);
                self.cmp(&d)
            },
            (Data::List(my_list), Data::List(their_list)) => {
                // If they're both lists, order by the first thing that differs
                let len_to_use = std::cmp::min(my_list.len(), their_list.len());
                for i in 0..len_to_use {
                    let mine = &my_list[i];
                    let theirs = &their_list[i];

                    match mine.cmp(theirs) {
                        Ordering::Equal => (), // continue to the next element
                        // @: name for the entire pattern (in this case Less and Greater)
                        ordering @ _ => return ordering, // break with ordering
                    }
                }
                // if we got here, see which one ends first
                my_list.len().cmp(&their_list.len())
            },
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_data(input: &str) -> (Data, &str) {
    if input.starts_with("[") {
        let mut list = vec![];

        let mut rest_of_str = &input[1..];
        while !rest_of_str.is_empty() {
            let b = rest_of_str.as_bytes()[0];
            match b {
                // finished the list: break out of the loop
                b']' => {
                    rest_of_str = &rest_of_str[1..];
                    break;
                },
                // starting a new list
                b'[' => {
                    let (new_list, returned_str) = parse_data(rest_of_str);
                    list.push(new_list);
                    rest_of_str = returned_str;
                }
                // comma: move on to the next thing
                b',' => {
                    rest_of_str = &rest_of_str[1..];
                },
                // otherwise, it's a number
                _ => {
                    let (num, returned_str) = parse_data(rest_of_str);
                    list.push(num);
                    rest_of_str = returned_str;
                },
            }
        }
        
        (Data::List(list), rest_of_str)
    }
    else {
        // it's a number: we need to get the string up to the next "," or "]",
        // and then parse it
        let mut index = None;
        for (i, b) in input.bytes().enumerate() {
            if b == b']' || b == b',' {
                index = Some(i);
                break;
            }
        }
        let index = index.unwrap_or(input.len());
        let num: usize = input[..index].parse().unwrap();
        (Data::Number(num), &input[index..])
    }
}


#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<Vec<(Data, Data)>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<(Data, Data)>> {
    let mut pairs = vec![];

    let v: Vec<&str> = input.lines().collect();

    // Input is pairs of "packets", separated by blank lines
    for pair in v.chunks(3) {
        let (left, left_remainder) = parse_data(pair[0]);
        let (right, right_remainder) = parse_data(pair[1]);
        
        assert!(left_remainder.is_empty());
        assert!(right_remainder.is_empty());

        pairs.push((left,right));
    }

    Ok(pairs)
}



// Part 1: sum of the indices of the pairs that are in the right order
// 1 indexed
#[aoc(day13, part1)]
pub fn solve_part1(input: &[(Data, Data)]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[(Data, Data)]) -> usize {
    

    let mut index_sum = 0;
    for (i, (left, right)) in input.iter().enumerate() {
        match left.cmp(right)
        {
            Ordering::Less => index_sum += i + 1,
            _ => ()
        }
    }

    index_sum
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &[(Data, Data)]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[(Data, Data)]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
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

        assert_eq!(result, 0);
    }

    // a helper for handwriting Data
    use super::Data;
    fn make_list(numbers: &[usize]) -> Data {
        let v = numbers.iter().map(|&x| Data::Number(x)).collect();
        Data::List(v)
    }
    // handwritten input to test things because parsing is hard
    #[test]
    fn test_handwritten() {
        let mut example = vec![];
        // hardcoding example just so I can test
        example.push((make_list(&[1,1,3,1,1]), make_list(&[1,1,5,1,1])));
        example.push((Data::List(vec![make_list(&[1]), make_list(&[2,3,4])]),
                    Data::List(vec![make_list(&[1]), Data::Number(4)])));
        example.push((make_list(&[9]), Data::List(vec![make_list(&[8,7,6])])));
        example.push((Data::List(vec![make_list(&[4,4,]), Data::Number(4), Data::Number(4)]),
        Data::List(vec![make_list(&[4,4,]), Data::Number(4), Data::Number(4), Data::Number(4)])));

        let result = super::solve_part1(&example);

        assert_eq!(result, 7);
    }
}