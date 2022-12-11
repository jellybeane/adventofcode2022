use std::{collections::VecDeque, rc::Rc};

use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::Result;

#[derive(Clone)]
pub struct Monkey {
    items: VecDeque<usize>,
    // the operation that this monkey does
    // a "reference-counted closure trait object"
    op: Rc<dyn Fn(usize) -> usize>,
    pub divisor: usize, 
    true_target: usize, // If divisible, throw here
    false_target: usize // If not divisible, throw here
}

impl Monkey {
    /// Add an item to my items
    pub fn add_item(&mut self, item: usize) {
        self.items.push_back(item);
    }

    /// inspect all of my items, and return who to throw each item to
    pub fn process_items_1(&mut self) -> Result<Vec<(usize, usize)>> {
        let mut thrown_items = vec![];
        while !self.items.is_empty() {
            // Get out the item
            let worry_level = self.items.pop_front().unwrap();
            // Inspect the item
            let worry_level = (self.op)(worry_level);
            // Get bored with the item
            let worry_level = worry_level / 3;
            // Throw to someone else
            let target = if worry_level % self.divisor == 0 {
                self.true_target
            }
            else {
                self.false_target
            };
            thrown_items.push((target, worry_level));
        }
        Ok(thrown_items)
    }

    
    // Part 2: worry levels are no longer divided by 3
    // Need to deal with unmanageably large numbers
    pub fn process_items_2(&mut self, product_of_divisors: usize) -> Result<Vec<(usize, usize)>> {
        let mut thrown_items = vec![];
        while !self.items.is_empty() {
            // Get out the item
            let mut worry_level = self.items.pop_front().unwrap();
            // Inspect the item
            worry_level = (self.op)(worry_level);

            // Throw to someone else
            let target = if worry_level % self.divisor == 0 {
                self.true_target
            }
            else {
                self.false_target
            };

            // We only care about divisibility 
            worry_level = if worry_level % product_of_divisors == 0 {
                product_of_divisors
            }
            else {
                worry_level % product_of_divisors
            };
            
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
        // line 1: items
        let (_, items_str) = monkey_lines[1].split_once(": ").unwrap();
        let items: VecDeque<usize> = items_str.split(", ").map(|x| {x.parse().unwrap()}).collect();
        
        // line 2: operation
        let (_, operation_str) = monkey_lines[2].split_once("old ").unwrap();
        let (operator_str, num_str) = operation_str.split_once(" ").unwrap();
        let opnum: Option<usize> = match num_str.parse() {
            Ok(val) => Some(val),
            Err(_) => None,
        };
        let op: Rc<dyn Fn(usize) -> usize> = match operator_str {
            "*" => {
                let closure = move |x| {
                    match opnum {
                        Some(val) => x * val,
                        None => x * x,
                    }
                };
                Rc::new(closure)
            },
            "+" => {
                let closure = move |x| {
                    match opnum {
                        Some(val) => x + val,
                        None => x + x,
                    }
                };
                Rc::new(closure)
            },
            _ => unreachable!()
        };

        // line 3: test (always divisible by)
        let (_, divisor_str) = monkey_lines[3].split_once("by ").unwrap();
        let divisor: usize = divisor_str.parse()?;

        // line 4: target monkey if true
        let (_, target_str) = monkey_lines[4].split_once("monkey ").unwrap();
        let true_target: usize = target_str.parse()?;
        // line 5: target monkey if true
        let (_, target_str) = monkey_lines[5].split_once("monkey ").unwrap();
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
    let mut monkeys = input.to_vec();
    let mut inspections = vec![0; monkeys.len()];
    // Part 1: monkey business after 20 rounds
    for _round in 0..20 {
        // for each monkey
        for index in 0..monkeys.len() {
            // the monkey does its thing, and returns all the thrown items
            let thrown_items = monkeys[index].process_items_1().unwrap();
            inspections[index] += thrown_items.len();
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
    let mut monkeys = input.to_vec();
    let mut inspections = vec![0; monkeys.len()];

    // hmm all the divisors are prime numbers?
    let divisors: Vec<usize> = monkeys.iter().map(|m| m.divisor).collect();
    let mut product_of_divisors = 1;
    for d in &divisors {
        product_of_divisors *= d;
    }

    // Part 2: monkey business after 10_000 rounds
    for _round in 0..10_000 {
        // for each monkey
        for index in 0..monkeys.len() {
            // the monkey does its thing, and returns all the thrown items
            let thrown_items = monkeys[index].process_items_2(product_of_divisors).unwrap();
            inspections[index] += thrown_items.len();
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

        assert_eq!(result, 2_713_310_158);
    }
}