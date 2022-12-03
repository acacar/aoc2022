
use aoc2022::{check_or_get_input, read_lines};

fn main () {
    let filepath = check_or_get_input(1).expect("Could not get input!");
    let groups = common(&filepath);
    println!("Part 1 result: {}", groups[0]);
    println!("Part 1 result: {}", groups[0..3].iter().fold(0, |acc, x| acc + x));
}

fn parse_input(filepath: &str) {
    
}

fn part1(filepath: &str) -> Vec<i32> {
    let elves_vec: Vec<Option<i32>> = read_lines(filepath).unwrap().map(|i| {match i.unwrap().as_str() { "" => None, s => Some(s.parse::<i32>().unwrap())}}).collect();
    let mut tmp = vec![];
    let mut total = 0;
    for i in elves_vec {
        if i == None {
            tmp.push(total);
            total = 0
        } else {
            total += i.unwrap();
        }
    }
    tmp.sort_unstable_by(|a, b| b.cmp(a));
    tmp
}