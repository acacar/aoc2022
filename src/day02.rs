use crate::check_or_get_input;

fn prep(input: &str) -> Vec<(String, String)> {
    let mut data = vec![];
    for line in input.lines() {
        let x: Vec<&str> = line.split(' ').collect();
        data.push((x[0].to_string(), x[1].to_string()));
    }
    data
}

#[derive(Clone)]
enum Play {
    Rock = 0,
    Paper = 1,
    Scissors = 2,
}

impl Play {
    fn from_i32(i: i32) -> Self {
        match i {
            0 => Play::Rock,
            1 => Play::Paper,
            2 => Play::Scissors,
            _ => panic!("Cannot convert {} to a play!", i)
        }
    }
}

fn elf_play(p1: &String) -> Play {
    match p1.as_str() {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => panic!("Bad input on first column!"),
    }
}

fn score(p1: &Play, p2: &Play) -> i32 {
    let elf = p1.clone() as i32;
    let me = p2.clone() as i32;
    match (me - elf).rem_euclid(3)
    {
        0 => (me + 1) + 3, // Draw
        1 => (me + 1) + 6, // Win
        2 => me + 1,       // Loss
        _ => panic!("Thissss: {} {} {}", me, elf, (me - elf) % 3)
    }
}

fn p1_score(p1: &String, p2: &String) -> i32 {

    let p2 = match p2.as_str() {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => panic!("Bad input on second column!"),
    };
    score(&elf_play(p1), &p2)
}

fn part1(data: &Vec<(String, String)>) -> i32 {
    data.iter().map(|(p1, p2)| p1_score(p1, p2)).sum()
}


fn p2_score(p1: &String, p2: &String) -> i32 {
    let elf = elf_play(p1);
    let me = match p2.as_str() {
        "X" => Play::from_i32((elf.clone() as i32 - 1).rem_euclid(3)),
        "Y" => Play::from_i32((elf.clone() as i32).rem_euclid(3)),
        "Z" => Play::from_i32((elf.clone() as i32 + 1).rem_euclid(3)),
        _ => panic!("Bad input on second column!"),
    };
    score(&elf, &me)
}

fn part2(data: &Vec<(String, String)>) -> i32 {
    data.iter().map(|(p1, p2)| p2_score(p1, p2)).sum()
}

pub fn solve() {
    let filename: String = check_or_get_input(2);
    let data = prep(std::fs::read_to_string(filename).expect("Day 2: cannot read input").as_str());
    println!("Day 2, part 1: {}", part1(&data));
    println!("Day 2, part 2: {}", part2(&data));
}

// TESTS

#[allow(dead_code)]
const DAY2_EXAMPLE: &str = r#"A Y
B X
C Z"#;

#[test]
fn test_day02_prep() {
    assert_eq!(prep(DAY2_EXAMPLE), vec![(String::from("A"), String::from("Y")),
                                        (String::from("B"), String::from("X")),
                                        (String::from("C"), String::from("Z"))]);
}

#[test]
fn test_day02_part1() {
    assert_eq!(part1(&prep(DAY2_EXAMPLE)), 15);
}

#[test]
fn test_day02_part2() {
    assert_eq!(part2(&prep(DAY2_EXAMPLE)), 12);
}
