use sscanf::sscanf;
use std::{collections::HashSet, time::Instant};

use crate::check_or_get_input;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse_from(c: char) -> Result<Direction, ()> {
        match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

fn prep(input: &str) -> Vec<(Direction, i32)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let (direction, distance) = sscanf!(line, "{} {}", char, i32).unwrap();
        result.push((Direction::parse_from(direction).unwrap(), distance));
    }
    result
}

fn move_end(direction: &Direction, pos: (i32, i32)) -> (i32, i32) {
    match direction {
        Direction::Up => (pos.0, pos.1 + 1),
        Direction::Down => (pos.0, pos.1 - 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn adjust_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    //Still touching
    if (head.0 - tail.0).abs() < 2 && (head.1 - tail.1).abs() < 2 {
        return tail;
    }
    match (head.0 - tail.0, head.1 - tail.1) {
        (0, 2) => (tail.0, tail.1 + 1),
        (0, -2) => (tail.0, tail.1 - 1),
        (2, 0) => (tail.0 + 1, tail.1),
        (-2, 0) => (tail.0 - 1, tail.1),
        (1, 2) => (tail.0 + 1, tail.1 + 1),
        (1, -2) => (tail.0 + 1, tail.1 - 1),
        (-1, 2) => (tail.0 - 1, tail.1 + 1),
        (-1, -2) => (tail.0 - 1, tail.1 - 1),
        (-2, 1) => (tail.0 - 1, tail.1 + 1),
        (-2, -1) => (tail.0 - 1, tail.1 - 1),
        (2, 1) => (tail.0 + 1, tail.1 + 1),
        (2, -1) => (tail.0 + 1, tail.1 - 1),
        (-2, -2) => (tail.0 - 1, tail.1 - 1),
        (2, -2) => (tail.0 + 1, tail.1 - 1),
        (-2, 2) => (tail.0 - 1, tail.1 + 1),
        (2, 2) => (tail.0 + 1, tail.1 + 1),
        _ => panic!(
            "Cannot follow preceding knot! head: {:?}, tail: {:?}",
            head, tail
        ),
    }
}

fn part1(input: &Vec<(Direction, i32)>) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut tail = (0, 0);
    let mut head = (0, 0);
    visited.insert(tail);
    for (direction, distance) in input {
        for _ in 0..*distance {
            head = move_end(direction, head);
            tail = adjust_tail(head, tail);
            visited.insert(tail);
        }
    }
    visited.len() as i32
}

fn part2(input: &Vec<(Direction, i32)>) -> i32 {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut knots = vec![(0, 0); 10];
    for (direction, distance) in input {
        for _ in 0..*distance {
            knots[0] = move_end(direction, knots[0]);
            for i in 1..10 {
                knots[i] = adjust_tail(knots[i - 1], knots[i]);
            }
            visited.insert(knots[9]);
        }
    }
    visited.len() as i32
}

pub fn solve() {
    let filename: String = check_or_get_input(9);
    let rope = prep(
        std::fs::read_to_string(filename)
            .expect("Day 9: cannot read input")
            .as_str(),
    );
    let mut st = Instant::now();
    let mut tmp = part1(&rope);
    println!(
        "Day 9, part1: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
    st = Instant::now();
    tmp = part2(&rope);
    println!(
        "Day 9, part2: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
}

// TESTS

#[allow(dead_code)]
const DAY09_EXAMPLE_1: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

#[allow(dead_code)]
const DAY09_EXAMPLE_2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;

#[test]
fn test_day09_prep() {
    assert_eq!(
        format!("{:?}", prep(DAY09_EXAMPLE_1)),
        "[(Right, 4), (Up, 4), (Left, 3), (Down, 1), (Right, 4), (Down, 1), (Left, 5), (Right, 2)]"
    );
}
#[test]
fn test_day09_part1() {
    assert_eq!(13, part1(&prep(DAY09_EXAMPLE_1)));
    assert_eq!(88, part1(&prep(DAY09_EXAMPLE_2)));
}

#[test]
fn test_day09_part2() {
    assert_eq!(36, part2(&prep(DAY09_EXAMPLE_2)));
}
