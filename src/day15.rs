use crate::check_or_get_input;
use sscanf::sscanf;
use std::{collections::HashSet, time::Instant};

fn prep(input: &str) -> Vec<(i64, i64, i64, i64)> {
    let mut locations: Vec<(i64, i64, i64, i64)> = vec![];
    for line in input.lines() {
        let location = sscanf!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i64,
            i64,
            i64,
            i64
        )
        .unwrap();
        locations.push(location);
    }
    locations
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn intersection(&self, other: &Range) -> Option<Range> {
        if self.end < other.start || other.end < self.start {
            None
        } else {
            Some(Range {
                start: std::cmp::max(self.start, other.start),
                end: std::cmp::min(self.end, other.end),
            })
        }
    }

    fn size(&self) -> i64 {
        self.end - self.start + 1
    }
}

fn collide(r1: &Range, r2: &Range) -> (Option<Range>, Option<Range>) {
    match r1.intersection(r2) {
        None => (Some(r1.clone()), Some(r2.clone())),
        Some(x) => {
            if x == *r1 {
                (None, Some(r2.clone()))
            } else if x == *r2 {
                (Some(r1.clone()), None)
            } else {
                if r1.start < r2.start {
                    (
                        Some(Range {
                            start: r1.start,
                            end: x.start - 1,
                        }),
                        Some(Range {
                            start: x.start,
                            end: r2.end,
                        }),
                    )
                } else {
                    (
                        Some(Range {
                            start: x.start,
                            end: r1.end,
                        }),
                        Some(Range {
                            start: r2.start,
                            end: x.start - 1,
                        }),
                    )
                }
            }
        }
    }
}

fn part1(sensors: &Vec<(i64, i64, i64, i64)>, at_line: i64) -> i64 {
    get_projections(sensors, at_line, None)
}

fn get_projections(
    sensors: &Vec<(i64, i64, i64, i64)>,
    at_line: i64,
    limits: Option<(i64, i64)>,
) -> i64 {
    let mut projections: Vec<Range> = vec![];
    let mut beacons_on_line: HashSet<(i64, i64)> = HashSet::new();
    for sensor in sensors.iter() {
        //println!("{:?}", sensor);
        let (sx, sy, bx, by) = sensor;
        if *by == at_line {
            if limits.is_some() {
                if !(*bx < limits.unwrap().0 || *bx > limits.unwrap().1) {
                    beacons_on_line.insert((*bx, *by));
                }
            } else {
                beacons_on_line.insert((*bx, *by));
            }
        }
        let min_radius = (sx - bx).abs() + (sy - by).abs();
        let dist_to_line = (at_line - sy).abs();
        if dist_to_line > min_radius {
            continue;
        } else {
            let proj_size = min_radius - dist_to_line;
            let mut range = Some(Range {
                start: if limits.is_some() && ((sx - proj_size) < limits.unwrap().0) {
                    limits.unwrap().0
                } else {
                    sx - proj_size
                },
                end: if limits.is_some() && ((sx + proj_size) > limits.unwrap().1) {
                    limits.unwrap().1
                } else {
                    sx + proj_size
                },
            });
            //println!("{:?}", range);
            let mut tmp_projections: Vec<Range> = vec![];
            while !projections.is_empty() && range.is_some() {
                let other = projections.pop().unwrap();
                match collide(&range.unwrap(), &other) {
                    (Some(x), Some(y)) => {
                        range = Some(x).clone();
                        tmp_projections.push(y);
                    }
                    (Some(x), None) => {
                        range = Some(x).clone();
                    }
                    (None, Some(y)) => {
                        tmp_projections.push(y);
                        while !projections.is_empty() {
                            tmp_projections.push(projections.pop().unwrap());
                        }
                        range = None;
                    }
                    (None, None) => {
                        panic!("Should not happen!")
                    }
                }
            }
            if range.is_some() {
                tmp_projections.push(range.unwrap().clone());
            }
            projections = tmp_projections;
        }
    }
    let mut total = 0;
    for p in projections {
        total += p.size();
    }
    total //-  beacons_on_line.len() as i64
}

#[allow(unused_variables)]
fn part2(sensors: &Vec<(i64, i64, i64, i64)>, limits: (i64, i64)) -> i64 {
    for line in limits.0..=limits.1 {
        let impossible = get_projections(sensors, line, Some(limits));

        if impossible < limits.1 - limits.0 + 1 {
            for x in limits.0..=limits.1 {
                let mut in_range = true;
                for s in sensors.iter() {
                    let (sx, sy, bx, by) = s;
                    let min_radius = (sx - bx).abs() + (sy - by).abs();
                    let dist_to_point = (x - sx).abs() + (line - sy).abs();
                    if dist_to_point <= min_radius {
                        in_range = false;
                        break;
                    }
                }
                if in_range {
                    return x * 4000000 + line;
                }
            }
        }
    }
    panic!("Houston, we have a problem!");
}

pub fn solve() {
    let filename: String = check_or_get_input(15);
    let locations = prep(
        std::fs::read_to_string(filename)
            .expect("Day 15: cannot read input")
            .as_str(),
    );
    let mut st = Instant::now();
    let tmp = part1(&locations, 2000000);
    println!(
        "Day 15, part1: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
    st = Instant::now();
    let tmp = part2(&locations, (0, 4000000));
    println!(
        "Day 15, part2: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
}

#[test]
fn test_day15_prep() {
    assert_eq!(
        vec![
            (2, 18, -2, 15),
            (9, 16, 10, 16),
            (13, 2, 15, 3),
            (12, 14, 10, 16),
            (10, 20, 10, 16),
            (14, 17, 10, 16),
            (8, 7, 2, 10),
            (2, 0, 2, 10),
            (0, 11, 2, 10),
            (20, 14, 25, 17),
            (17, 20, 21, 22),
            (16, 7, 15, 3),
            (14, 3, 15, 3),
            (20, 1, 15, 3)
        ],
        prep(DAY15_EXAMPLE)
    );
}

#[test]
// [7, 1, 3, 4, 4, 5, 9, 10, 3, 8, 6, 5, 1, 7]
fn test_day15_part1() {
    assert_eq!(part1(&prep(DAY15_EXAMPLE), 10), 26);
}

#[test]
fn test_day15_part2() {
    println!("{}", part2(&prep(DAY15_EXAMPLE), (0, 20)));
}

#[allow(dead_code)]
const DAY15_EXAMPLE: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

#[allow(dead_code)]
const DAY15_EXAMPLE_2: &str = r#"Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10"#;
