use std::{collections::VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Operation {
    // The operation this monkey applies
    // None indicates the given value should be reused
    Add(Option<usize>),
    Multiply(Option<usize>)
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Monkey {
    items: VecDeque<usize>,
    op: Operation,
    divisor: usize, 
    true_target: usize, // If divisible, throw here
    false_target: usize // If not divisible, throw here
}

impl Monkey {
    pub fn num_items(&self) -> usize {
        self.items.len()
    }

    // any more items?
    pub fn has_next(&self) -> bool {
        !self.items.is_empty()
    }

    /// Get out the item to inspect, or None if there are no more
    pub fn next_item(&mut self) -> Option<usize> {
        self.items.pop_front()
    }

    /// Add an item to my items
    pub fn add_item(&mut self, item: usize) {
        self.items.push_back(item);
    }

    /// Return the new worry level after applying my operation
    pub fn apply_operation(&mut self, item: usize) -> usize {
        match self.op {
            Operation::Add(Some(val)) => item + val,
            Operation::Add(None) => item + item,
            Operation::Multiply(Some(val)) => item * val,
            Operation::Multiply(None) => item * item,
        }
    }

    /// Who should this item be thrown to?
    pub fn get_target(&self, item: usize) -> usize {
        if item % self.divisor == 0 {
            self.true_target
        }
        else {
            self.false_target
        }
    }

    // inspect all of my items, and return who to throw each item to
    pub fn process_items(&mut self) -> Result<Vec<(usize, usize)>> {
        let mut thrown_items = vec![];
        while self.has_next() {
            // Get out the item
            let worry_level = self.next_item().unwrap();
            // Inspect the item
            let worry_level = self.apply_operation(worry_level);
            // Get bored with the item
            let worry_level = worry_level / 3;
            // Throw to someone else
            let target = self.get_target(worry_level);
            thrown_items.push((target, worry_level));
        }
        Ok(thrown_items)
    }
}

// Input is monkeys, each formatted like so:
// Monkey 0:
//   Starting items: 61
//   Operation: new = old * 11
//   Test: divisible by 5
//     If true: throw to monkey 7
//     If false: throw to monkey 4
#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Result<Vec<Monkey>> {
    input_generator_inner(input)
}
fn input_generator_inner(input: &str) -> Result<Vec<Monkey>> {
    let mut monkeys = vec!();

    let v: Vec<&str> = input.lines().collect();
    // doing cursed things because I know the structure
    // each monkey's text is 6 lines plus a whitespace
    for monkey_lines in v.chunks(7) {
        //dbg!(monkeys.len());
        // line 1: items
        let (_, items_str) = monkey_lines[1].split_once(": ").unwrap();
        //dbg!(items_str);
        let items: VecDeque<usize> = items_str.split(", ").map(|x| {x.parse().unwrap()}).collect();
        
        // line 2: operation
        let (_, operation_str) = monkey_lines[2].split_once("old ").unwrap();
        //dbg!(operation_str);
        let (operator_str, num_str) = operation_str.split_once(" ").unwrap();
        let opnum = match  num_str.parse() {
            Ok(val) => Some(val),
            Err(_) => None,
        };
        let op = match operator_str {
            "*" => Operation::Multiply(opnum),
            "+" => Operation::Add(opnum),
            _ => unreachable!()
        };

        // line 3: test (always divisible by)
        let (_, divisor_str) = monkey_lines[3].split_once("by ").unwrap();
        //dbg!(divisor_str);
        let divisor: usize = divisor_str.parse()?;

        // line 4: target monkey if true
        let (_, target_str) = monkey_lines[4].split_once("monkey ").unwrap();
        //dbg!(target_str);
        let true_target: usize = target_str.parse()?;
        // line 5: target monkey if true
        let (_, target_str) = monkey_lines[5].split_once("monkey ").unwrap();
        //dbg!(target_str);
        let false_target: usize = target_str.parse()?;

        let monkey = Monkey {items, op, divisor, true_target, false_target};
        monkeys.push(monkey);
    }
    Ok(monkeys)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[Monkey]) -> usize {
    solve_part1_inner(input)
}
fn solve_part1_inner(input: &[Monkey]) -> usize {
    //dbg!(input);

    let mut monkeys = input.to_vec();
    let mut inspections = vec![0; monkeys.len()];
    // Part 1: monkey business after 20 rounds
    for _round in 0..20 {
        // for each monkey
        // TODO I don't understand how to iterate over monkeys
        for index in 0..monkeys.len() {
            // the monkey does its thing, and returns all the thrown items
            inspections[index] += monkeys[index].num_items();
            let thrown_items = monkeys[index].process_items().unwrap();
            // all thrown items are received by target monkeys
            for (target, item) in thrown_items {
                monkeys[target].add_item(item);
            }
        }
    }

    // Find the two monkeys with the most inspections
    // and multiply together the number of items they inspected
    inspections.sort();
    let mut monkey_it = inspections.iter().rev();
    let most_active = monkey_it.next().unwrap();
    let next_active = monkey_it.next().unwrap();
    most_active * next_active

}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[Monkey]) -> usize {
    solve_part2_inner(input)
}
fn solve_part2_inner(input: &[Monkey]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &'static str =
r#"Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
  If true: throw to monkey 2
  If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
  If true: throw to monkey 2
  If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
  If true: throw to monkey 1
  If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
  If true: throw to monkey 0
  If false: throw to monkey 1
"#;
    #[test]
    fn test_part1_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part1(&input);

        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part2_example() {
        let input = super::input_generator(TEST_INPUT).unwrap();
        let result = super::solve_part2(&input);

        assert_eq!(result, 1);
    }
}