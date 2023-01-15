use std::{cell::RefCell, time::Instant};

use aoc2022::check_or_get_input;
use sscanf::sscanf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
    Add(usize),
    Multiply(usize),
    Square,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: RefCell<Vec<usize>>,
    operation: Op,
    throw_true: usize,
    throw_false: usize,
    modulus: usize,
}

fn prep(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_id: Option<usize> = None;
    let mut monkey_items: Option<Vec<usize>> = None;
    let mut monkey_operation: Option<Op> = None;
    let mut monkey_modulus: Option<usize> = None;
    let mut monkey_true_throw: Option<usize> = None;
    let mut monkey_false_throw: Option<usize> = None;
    for line in input.lines() {
        match line.trim() {
            line if line.is_empty() => continue,
            line if line.contains("Monkey") => {
                if monkey_id.is_some() {
                    let monkey = Monkey {
                        items: RefCell::new(monkey_items.unwrap()),
                        operation: monkey_operation.unwrap(),
                        throw_true: monkey_true_throw.unwrap(),
                        throw_false: monkey_false_throw.unwrap(),
                        modulus: monkey_modulus.unwrap(),
                    };
                    monkeys.push(monkey);
                }
                monkey_id = Some(sscanf!(line, "Monkey {}:", usize).unwrap());
                monkey_items = None;
                monkey_operation = None;
                monkey_modulus = None;
                monkey_true_throw = None;
                monkey_false_throw = None;
            }
            line if line.contains("Starting items:") => {
                let items: Vec<usize> = line
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .map(|x| x.parse().unwrap())
                    .collect();
                monkey_items = Some(items);
            }
            line if line.contains("Operation:") => {
                let tmp = sscanf!(line, "Operation: new = old {} {}", String, String).unwrap();

                monkey_operation = if tmp.1 == "old" {
                    if tmp.0 == "+" {
                        Some(Op::Multiply(2))
                    } else if tmp.0 == "*" {
                        Some(Op::Square)
                    } else {
                        panic!("Unexpected operation: {}", line)
                    }
                } else {
                    if tmp.0 == "+" {
                        Some(Op::Add(tmp.1.parse().unwrap()))
                    } else if tmp.0 == "*" {
                        Some(Op::Multiply(tmp.1.parse().unwrap()))
                    } else {
                        panic!("Unexpected operation: {}", line)
                    }
                }
            }
            line if line.contains("Test") => {
                let x = sscanf!(line, "Test: divisible by {}", usize).unwrap();
                monkey_modulus = Some(x);
            }
            line if line.contains("If true") => {
                let x = sscanf!(line, "If true: throw to monkey {}", usize).unwrap();
                monkey_true_throw = Some(x);
            }
            line if line.contains("If false") => {
                let x = sscanf!(line, "If false: throw to monkey {}", usize).unwrap();
                monkey_false_throw = Some(x);
            }
            _ => {
                panic!("Unexpected line: {}", line)
            }
        }
    }
    if monkey_id.is_some() {
        let monkey = Monkey {
            items: RefCell::new(monkey_items.unwrap()),
            operation: monkey_operation.unwrap(),
            throw_true: monkey_true_throw.unwrap(),
            throw_false: monkey_false_throw.unwrap(),
            modulus: monkey_modulus.unwrap(),
        };
        monkeys.push(monkey);
    }
    monkeys
}

fn part1(input: &mut Vec<Monkey>) -> usize {
    let mut num_inspected: Vec<usize> = vec![0; input.len()];
    for _ in 0..20 {
        for (idx, monkey) in input.iter().enumerate() {
            for item in monkey.items.borrow().iter() {
                num_inspected[idx] += 1;
                let newitem = (match monkey.operation {
                    Op::Add(x) => item + x,
                    Op::Multiply(x) => item * x,
                    Op::Square => item * item,
                }) / 3;
                let target = if newitem % monkey.modulus == 0 {
                    monkey.throw_true.clone()
                } else {
                    monkey.throw_false.clone()
                };
                input[target].items.borrow_mut().push(newitem.clone());
            }
            monkey.items.borrow_mut().clear();
        }
    }

    num_inspected.sort_by(|a, b| b.cmp(a));
    num_inspected[0] * num_inspected[1]
}

fn part2(input: &mut Vec<Monkey>) -> usize {
    let mut num_inspected: Vec<usize> = vec![0; input.len()];
    let modulo_lcm: usize = input.iter().map(|m| m.modulus).product();
    for _ in 0..10000 {
        for (idx, monkey) in input.iter().enumerate() {
            for item in monkey.items.borrow().iter() {
                num_inspected[idx] += 1;
                let newitem = (match monkey.operation {
                    Op::Add(x) => item + x,
                    Op::Multiply(x) => item * x,
                    Op::Square => item * item,
                }) % modulo_lcm;
                let target = if newitem % monkey.modulus == 0 {
                    monkey.throw_true.clone()
                } else {
                    monkey.throw_false.clone()
                };
                input[target].items.borrow_mut().push(newitem.clone());
            }
            monkey.items.borrow_mut().clear();
        }
    }

    num_inspected.sort_by(|a, b| b.cmp(a));
    num_inspected[0] * num_inspected[1]
}

pub fn main() {
    let filename: String = check_or_get_input(11);
    let monkeys = prep(
        std::fs::read_to_string(filename)
            .expect("Day 11: cannot read input")
            .as_str(),
    );
    let mut st = Instant::now();
    let mut tmp = part1(&mut monkeys.clone());
    println!(
        "Day 11, part1: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
    st = Instant::now();
    tmp = part2(&mut monkeys.clone());
    println!(
        "Day 11, part2: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
}

// TESTS

#[test]
fn test_day10_prep() {
    let testcase = prep(DAY11_EXAMPLE_1);
    assert_eq!(testcase.len(), 4);
    assert_eq!(
        testcase.iter().map(|m| m.modulus).collect::<Vec<usize>>(),
        vec![23, 19, 13, 17]
    );
    assert_eq!(
        testcase
            .iter()
            .map(|m| m.throw_true)
            .collect::<Vec<usize>>(),
        vec![2, 2, 1, 0]
    );
    assert_eq!(
        testcase
            .iter()
            .map(|m| m.throw_false)
            .collect::<Vec<usize>>(),
        vec![3, 0, 3, 1]
    );
    assert_eq!(
        testcase.iter().map(|m| m.operation).collect::<Vec<Op>>(),
        vec![Op::Multiply(19), Op::Add(6), Op::Square, Op::Add(3)]
    );
    assert_eq!(
        testcase
            .iter()
            .map(|m| m.items.borrow().len())
            .collect::<Vec<usize>>(),
        vec![2, 4, 3, 1]
    );
}
#[test]
fn test_day10_part1() {
    assert_eq!(10605, part1(&mut prep(DAY11_EXAMPLE_1)));
}

#[test]
fn test_day10_part2() {
    assert_eq!(2_713_310_158, part2(&mut prep(DAY11_EXAMPLE_1)));
}

#[allow(dead_code)]
const DAY11_EXPECTED_1: usize = 10605;

#[allow(dead_code)]
const DAY11_EXAMPLE_1: &str = r#"Monkey 0:
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
  If false: throw to monkey 1"#;
