use std::collections::{hash_map::RandomState, HashSet};

use crate::check_or_get_input;

fn prep(input: &str) -> Vec<String> {
    input.lines().map(|s| String::from(s)).collect()
}

fn find_common_in_compartments(s: &String) -> HashSet<char> {
    let mid = s.len() / 2;
    let compartment1: HashSet<char, RandomState> = HashSet::from_iter(s[0..mid].chars());
    let compartment2: HashSet<char, RandomState> = HashSet::from_iter(s[mid..].chars());
    return compartment1
        .intersection(&compartment2)
        .into_iter()
        .map(|c| c.to_owned())
        .collect();
}

fn find_common_in_groups(v: &[String]) -> HashSet<char> {
    let elf1: HashSet<char, RandomState> = HashSet::from_iter(v[0].chars());
    let elf2: HashSet<char, RandomState> = HashSet::from_iter(v[1].chars());
    let elf3: HashSet<char, RandomState> = HashSet::from_iter(v[2].chars());
    let tmp = HashSet::from_iter(elf1.intersection(&elf2).into_iter().map(|c| c.to_owned()));
    return tmp
        .intersection(&elf3)
        .into_iter()
        .map(|c| c.to_owned())
        .collect();
}

fn get_priority(c: &char) -> i32 {
    if c.clone() as i32 > 0x60 {
        return (c.clone() as i32) - 0x60;
    } else {
        return (c.clone() as i32 - 0x40) + 26;
    }
}

fn part1(data: &Vec<String>) -> i32 {
    let mut total = 0;
    for bag in data {
        for common in find_common_in_compartments(bag) {
            total += get_priority(&common);
        }
    }
    return total;
}

fn part2(data: &Vec<String>) -> i32 {
    let mut total = 0;
    for group in data.chunks(3) {
        for common in find_common_in_groups(group) {
            total += get_priority(&common)
        }
    }
    return total;
}

pub fn solve() {
    let filename: String = check_or_get_input(3);
    let data = prep(
        std::fs::read_to_string(filename)
            .expect("Day 3: cannot read input")
            .as_str(),
    );
    println!("Day 3, part 1: {}", part1(&data));
    println!("Day 3, part 2: {}", part2(&data));
}

// TESTS

#[allow(dead_code)]
const DAY3_EXAMPLE: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

#[test]
fn test_day03_prep() {
    assert_eq!(
        prep(DAY3_EXAMPLE),
        vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw"
        ]
    )
}

#[test]
fn test_day03_part1() {
    assert_eq!(part1(&prep(DAY3_EXAMPLE)), 157)
}

#[test]
fn test_day03_part2() {
    assert_eq!(part2(&prep(DAY3_EXAMPLE)), 70)
}
