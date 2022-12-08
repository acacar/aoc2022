use std::collections::HashMap;

use crate::check_or_get_input;

fn prep(input: &str) -> ((i32, i32), HashMap<(i32, i32), i32>) {
    let mut map = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut len_x = 0;
    for line in input.lines() {
        for c in line.chars() {
            let height: i32 = c.to_string().parse().expect("Invalid character");
            map.insert((x, y), height);
            x += 1;
        }
        len_x = x;
        y += 1;
        x = 0;
    }
    ((len_x, y), map)
}

fn part1(input: &((i32, i32), HashMap<(i32, i32), i32>)) -> i32 {
    let trees = &input.1;
    let len_x = input.0 .0;
    let len_y = input.0 .1;
    let mut num_visible = 0;
    for t in trees.keys() {
        if t.0 == 0 || t.0 == len_x - 1 || t.1 == 0 || t.1 == len_y - 1 {
            continue;
        }

        let mut vtop = true;
        let mut vbot = true;
        let mut vleft = true;
        let mut vright = true;

        for tx in 0..t.0 {
            let txo = (tx, t.1);
            if trees.get(&txo).unwrap() >= trees.get(t).unwrap() {
                vleft = false;
                break;
            }
        }
        for tx in t.0 + 1..len_x {
            let txo = (tx, t.1);
            if trees.get(&txo).unwrap() >= trees.get(t).unwrap() {
                vright = false;
                break;
            }
        }
        for ty in 0..t.1 {
            let tyo = (t.0, ty);
            if trees.get(&tyo).unwrap() >= trees.get(t).unwrap() {
                vtop = false;
                break;
            }
        }
        for ty in t.1 + 1..len_y {
            let tyo = (t.0, ty);
            if trees.get(&tyo).unwrap() >= trees.get(t).unwrap() {
                vbot = false;
                break;
            }
        }
        if vtop || vbot || vleft || vright {
            num_visible += 1;
        }
    }
    num_visible + (len_x * len_y - ((len_x - 2) * (len_y - 2)))
}

fn part2(input: &((i32, i32), HashMap<(i32, i32), i32>)) -> i32 {
    let trees = &input.1;
    let len_x = input.0 .0;
    let len_y = input.0 .1;
    let mut scenic_scores = vec![];
    for t in trees.keys() {
        let mut vtop = 0;
        let mut vbot = 0;
        let mut vleft = 0;
        let mut vright = 0;

        for tx in (0..t.0).rev() {
            let txo = (tx, t.1);
            vleft += 1;
            if trees.get(&txo).unwrap() >= trees.get(t).unwrap() {
                break;
            }
        }
        for tx in t.0 + 1..len_x {
            let txo = (tx, t.1);
            vright += 1;
            if trees.get(&txo).unwrap() >= trees.get(t).unwrap() {
                break;
            }
        }
        for ty in (0..t.1).rev() {
            let tyo = (t.0, ty);
            vtop += 1;
            if trees.get(&tyo).unwrap() >= trees.get(t).unwrap() {
                break;
            }
        }
        for ty in t.1 + 1..len_y {
            let tyo = (t.0, ty);
            vbot += 1;
            if trees.get(&tyo).unwrap() >= trees.get(t).unwrap() {
                break;
            }
        }
        scenic_scores.push(vbot * vtop * vleft * vright);
    }
    scenic_scores.sort();
    scenic_scores.last().unwrap().to_owned()
}

pub fn solve() {
    let filename: String = check_or_get_input(8);
    let trees = prep(
        std::fs::read_to_string(filename)
            .expect("Day 8: cannot read input")
            .as_str(),
    );
    println!("Day 08, part1: {}", part1(&trees));
    println!("Day 08, part2: {}", part2(&trees));
}

// TESTS

#[allow(dead_code)]
const DAY08_EXAMPLE: &str = r#"30373
25512
65332
33549
35390"#;

#[test]
fn test_day08_prep() {
    let ((_, _), trees) = prep(DAY08_EXAMPLE);
    assert_eq!(trees.get(&(0, 0)).unwrap().to_owned(), 3i32);
    assert_eq!(trees.get(&(1, 2)).unwrap().to_owned(), 5i32);
    assert_eq!(trees.get(&(3, 4)).unwrap().to_owned(), 9i32);
    assert_eq!(trees.get(&&(3, 3)).unwrap().to_owned(), 4i32);
    assert_eq!(trees.get(&&(0, 2)).unwrap().to_owned(), 6i32);
    assert_eq!(trees.get(&&(2, 2)).unwrap().to_owned(), 3i32);
    assert_eq!(trees.get(&&(4, 4)).unwrap().to_owned(), 0i32);
}
#[test]
fn test_day08_part1() {
    assert_eq!(21, part1(&prep(DAY08_EXAMPLE)));
}

#[test]
fn test_day06_part2() {
    assert_eq!(8, part2(&prep(DAY08_EXAMPLE)));
}
