use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    time::Instant,
};

use crate::check_or_get_input;

type Position = (i32, i32);
fn prep(input: &str) -> (Position, Position, HashMap<Position, u8>) {
    let mut map: HashMap<Position, u8> = HashMap::new();

    let mut y: i32 = 0;
    let mut spos: Option<Position> = None;
    let mut epos: Option<Position> = None;

    for line in input.lines() {
        let mut x: i32 = 0;
        for ch in line.chars() {
            if ch == 'S' {
                spos = Some((x, y));
                map.insert((x, y), 0);
            } else if ch == 'E' {
                epos = Some((x, y));
                map.insert((x, y), 25);
            } else {
                let elevation = (ch as u8) - 97;
                map.insert((x, y), elevation);
            }

            x += 1;
        }
        y += 1;
    }
    (spos.unwrap(), epos.unwrap(), map)
}

fn neighbors(pos: &Position, map: &HashMap<Position, u8>) -> Vec<Position> {
    let mut n: Vec<Position> = vec![];
    for dir in vec![(0, -1), (0, 1), (-1, 0), (1, 0)] {
        let newpos = (pos.0 + dir.0, pos.1 + dir.1);
        if map.contains_key(&newpos) {
            if map.get(&newpos).unwrap() <= &(map.get(&pos).unwrap() + 1) {
                n.push(newpos);
            }
        }
    }
    n
}

#[derive(Eq)]
struct Candidate {
    pos: Position,
    f_score: i32,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&other).f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

fn dist_fn(x: &Position, goal: &Position) -> i32 {
    (x.0 - goal.0).abs() + (x.1 - goal.1).abs()
}

fn reconstruct_path(path_map: HashMap<Position, Position>, n: Position) -> Vec<Position> {
    let mut retvec = Vec::new();
    retvec.push(n);
    let mut current = n.to_owned();
    while path_map.contains_key(&current) {
        current = path_map.get(&current).unwrap().to_owned();
        retvec.push(current);
    }
    retvec
}

fn astar(start: &Position, end: &Position, map: &HashMap<Position, u8>) -> Option<Vec<Position>> {
    let mut open_heap: BinaryHeap<Candidate> = BinaryHeap::new();
    let mut open_set: HashSet<Position> = HashSet::new();
    let mut came_from: HashMap<Position, Position> = HashMap::new();
    let mut g_score: HashMap<Position, i32> = HashMap::new();
    g_score.insert(start.to_owned(), 0);
    let mut f_score: HashMap<Position, i32> = HashMap::new();
    f_score.insert(start.to_owned(), dist_fn(start, end));

    open_heap.push(Candidate {
        pos: start.to_owned(),
        f_score: f_score.get(&start).unwrap() + g_score.get(&start).unwrap(),
    });
    open_set.insert(start.to_owned());
    while !open_heap.is_empty() {
        let c = open_heap.pop().unwrap();
        let current = c.pos.to_owned();
        open_set.remove(&current);
        if &current == end {
            return Some(reconstruct_path(came_from, current));
        }

        for n in neighbors(&current, &map).iter() {
            let t_g_score = match g_score.get(&current) {
                None => 99999999,
                Some(x) => x.to_owned(),
            } + 1;
            let n_g_score = match g_score.get(&n) {
                None => 99999999,
                Some(x) => x.to_owned(),
            };
            if t_g_score < n_g_score {
                came_from.insert(n.to_owned(), current);
                g_score.insert(n.to_owned(), t_g_score);
                f_score.insert(n.to_owned(), t_g_score + dist_fn(n, end));
                if !open_set.contains(n) {
                    open_set.insert(n.to_owned());
                    open_heap.push(Candidate {
                        pos: n.to_owned(),
                        f_score: f_score.get(&n).unwrap().to_owned(),
                    })
                }
            }
        }
    }
    None
}

fn part1(start: &Position, end: &Position, map: &HashMap<Position, u8>) -> usize {
    astar(&start, &end, &map).unwrap().len() - 1
}

fn part2(end: &Position, map: &HashMap<Position, u8>) -> usize {
    let mut min_score: usize = 99999999;
    for k in map.keys() {
        let v = map.get(k).unwrap().to_owned();
        if v == 0 {
            let path = astar(&k, &end, &map);
            if path.is_some() {
                let score = path.unwrap().len() - 1;
                if score < min_score {
                    min_score = score
                }
            }
        }
    }
    min_score
}

pub fn solve() {
    let filename: String = check_or_get_input(12);
    let (start, end, map) = prep(
        std::fs::read_to_string(filename)
            .expect("Day 12: cannot read input")
            .as_str(),
    );
    let mut st = Instant::now();
    let tmp = part1(&start, &end, &map);
    println!(
        "Day 12, part1: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
    st = Instant::now();
    let tmp = part2(&end, &map);
    println!(
        "Day 12, part2: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
}

#[test]
fn test_day12_prep() {
    println!("{:?}", prep(DAY12_EXAMPLE));
}

#[test]
fn test_day12_part1() {
    let (start, end, map) = prep(DAY12_EXAMPLE);
    println!("{}", part1(&start, &end, &map))
}

#[test]
fn test_day12_part2() {
    let (_, end, map) = prep(DAY12_EXAMPLE);
    println!("{}", part2(&end, &map))
}

#[allow(dead_code)]
const DAY12_EXAMPLE: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
