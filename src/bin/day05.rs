use std::collections::HashMap;

use aoc2022::check_or_get_input;
use sscanf::sscanf;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stack {
    crates: Vec<String>,
}

impl Stack {
    fn top(self: &Self) -> Option<&String> {
        self.crates.last()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    amount: i32,
    from: i32,
    to: i32,
}

fn separate_string_using_blank_line(s: &str) -> Vec<&str> {
    s.split("\n\n").collect()
}

fn stack_from(i: usize, lines: &Vec<String>) -> Stack {
    let mut retval = Stack { crates: Vec::new() };
    for line in lines.iter() {
        let position = (1 + 4 * (i - 1)) as usize;
        let crate_name = (line.as_bytes()[position] as char).to_string();
        if crate_name != " " {
            retval.crates.push(crate_name);
        }
    }
    retval.crates.pop();
    retval.crates.reverse();
    retval
}

fn parse_stacks(s: &str) -> HashMap<i32, Stack> {
    let lines: Vec<String> = s.lines().map(|l| String::from(l)).collect();
    let mut stacks: HashMap<i32, Stack> = HashMap::new();
    let numstacks = (lines.last().unwrap().len() + 1) / 4;
    for i in 1..(numstacks + 1) {
        stacks.insert(i as i32, stack_from(i, &lines));
    }
    stacks
}

fn parse_moves(s: &str) -> Vec<Move> {
    let mut retval: Vec<Move> = Vec::new();
    for line in s.lines() {
        let (amount, from, to) = sscanf!(line, "move {} from {} to {}", i32, i32, i32).unwrap();
        retval.push(Move { amount, from, to });
    }
    retval
}

fn prep(input: &str) -> (HashMap<i32, Stack>, Vec<Move>) {
    let input_parts = separate_string_using_blank_line(input);
    let crates_str = input_parts[0];
    let moves_str = input_parts[1];
    let stacks = parse_stacks(crates_str);
    let moves = parse_moves(moves_str);
    (stacks, moves)
}

fn move_btw_stacks_one_by_one(from: &mut Stack, to: &mut Stack, amount: i32) {
    for _ in 0..amount {
        let crate_name = from.crates.pop().unwrap();
        to.crates.push(crate_name);
    }
}

fn move_btw_stacks_all_at_once(from: &mut Stack, to: &mut Stack, amount: i32) {
    let mut tmp: Vec<String> = Vec::new();
    for _ in 0..amount {
        tmp.push(from.crates.pop().unwrap());
    }
    for _ in 0..amount {
        to.crates.push(tmp.pop().unwrap());
    }
}

fn part1(stacks: &HashMap<i32, Stack>, moves: &Vec<Move>) -> String {
    let mut stacks = stacks.clone();
    for m in moves {
        let from_idx = m.from as i32;
        let to_idx = m.to as i32;
        let mut from = stacks.get(&from_idx).unwrap().clone();
        let mut to = stacks.get(&to_idx).unwrap().clone();
        move_btw_stacks_one_by_one(&mut from, &mut to, m.amount);
        stacks.insert(from_idx, from);
        stacks.insert(to_idx, to);
    }
    let mut retval = String::new();
    for i in 1..=stacks.len() {
        let stack = stacks.get(&(i as i32)).unwrap();
        if stack.top().is_some() {
            retval.push_str(stack.top().unwrap());
        }
    }
    retval
}

fn part2(stacks: &HashMap<i32, Stack>, moves: &Vec<Move>) -> String {
    let mut stacks = stacks.clone();
    for m in moves {
        let from_idx = m.from as i32;
        let to_idx = m.to as i32;
        let mut from = stacks.get(&from_idx).unwrap().clone();
        let mut to = stacks.get(&to_idx).unwrap().clone();
        move_btw_stacks_all_at_once(&mut from, &mut to, m.amount);
        stacks.insert(from_idx, from);
        stacks.insert(to_idx, to);
    }
    let mut retval = String::new();
    for i in 1..=stacks.len() {
        let stack = stacks.get(&(i as i32)).unwrap();
        if stack.top().is_some() {
            retval.push_str(stack.top().unwrap());
        }
    }
    retval
}

pub fn main() {
    let filename: String = check_or_get_input(5);
    match prep(
        std::fs::read_to_string(filename)
            .expect("Day 5: cannot read input")
            .as_str(),
    ) {
        (stacks, moves) => {
            println!("Day 5, part 1: {}", part1(&stacks, &moves));
            println!("Day 5, part 2: {}", part2(&stacks, &moves));
        }
    }
}

// TESTS

#[allow(dead_code)]
const DAY5_EXAMPLE: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

#[allow(dead_code)]
fn subtest_day05_separate_string_using_blank_line() {
    let expected = vec![
        r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 "#,
        r#"move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#,
    ];
    assert_eq!(separate_string_using_blank_line(DAY5_EXAMPLE), expected);
}
#[allow(dead_code)]
fn subtest_day05_parse_stacks() {
    let s = separate_string_using_blank_line(DAY5_EXAMPLE)[0];
    let stacks = parse_stacks(s);
    let mut expected: HashMap<i32, Stack> = HashMap::new();

    expected.insert(
        1,
        Stack {
            crates: vec!["Z".to_string(), "N".to_string()],
        },
    );
    expected.insert(
        2,
        Stack {
            crates: vec!["M".to_string(), "C".to_string(), "D".to_string()],
        },
    );
    expected.insert(
        3,
        Stack {
            crates: vec!["P".to_string()],
        },
    );

    assert_eq!(stacks.len(), 3 as usize);
    for (idx, stack) in stacks {
        assert_eq!(stack, expected[&idx]);
    }
}

#[allow(dead_code)]
fn subtest_day05_parse_moves() {
    let s = separate_string_using_blank_line(DAY5_EXAMPLE)[1];
    let moves = parse_moves(s);
    let expected = vec![
        Move {
            amount: 1,
            from: 2,
            to: 1,
        },
        Move {
            amount: 3,
            from: 1,
            to: 3,
        },
        Move {
            amount: 2,
            from: 2,
            to: 1,
        },
        Move {
            amount: 1,
            from: 1,
            to: 2,
        },
    ];
    assert_eq!(moves.len(), 4 as usize);
    let mut i = 0;
    for m in moves {
        assert_eq!(m, expected[i]);
        i += 1;
    }
}

#[test]
fn test_day05_prep() {
    subtest_day05_separate_string_using_blank_line();
    subtest_day05_parse_moves();
    subtest_day05_parse_stacks();
}
#[test]
fn test_day05_part1() {
    let (stacks, moves) = prep(DAY5_EXAMPLE);
    assert_eq!(part1(&stacks, &moves), "CMZ");
}

#[test]
fn test_day05_part2() {
    let (stacks, moves) = prep(DAY5_EXAMPLE);
    assert_eq!(part2(&stacks, &moves), "MCD");
}
