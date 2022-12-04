use crate::check_or_get_input;

fn prep(input: &str) -> Vec<((i32, i32), (i32, i32))> {
    let mut data = vec![];
    for line in input.lines() {
        let mut regions = vec![];
        for region in line.split(",") {
            let mut bounds = vec![];
            for bound in region.split("-") {
                bounds.push(bound.parse::<i32>().expect("bad input"));
            }
            regions.push((bounds[0], bounds[1]));
        }
        data.push((regions[0], regions[1]));
    }
    data
}

fn contains(regions: &((i32, i32), (i32, i32))) -> bool {
    match regions {
        ((r1l, r1h), (r2l, r2h)) => (r1l <= r2l && r1h >= r2h) || (r2l <= r1l && r2h >= r1h)
    }
}


fn part1(data: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut count = 0;
    for regions in data {
        if contains(regions) {
            count += 1
        }
    }
    count
}

fn overlaps(regions: &((i32, i32), (i32, i32))) -> bool {
    match regions {
        ((r1l, r1h), (r2l, r2h)) => (r1l < r2l && r2l <= r1h) ||  ( r1l >= r2l && r1l <= r2h )
    }
}

fn part2(data: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut count = 0;
    for regions in data {
        if overlaps(regions) {
            count += 1
        }
    }
    count
}

pub fn solve() {
    let filename: String = check_or_get_input(4);
    let data = prep(
        std::fs::read_to_string(filename)
            .expect("Day 4: cannot read input")
            .as_str(),
    );
    println!("Day 4, part 1: {}", part1(&data));
    println!("Day 4, part 2: {}", part2(&data));
}

// TESTS

#[allow(dead_code)]
const DAY4_EXAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

#[test]
fn test_day04_prep() {
    assert_eq!(prep(DAY4_EXAMPLE), [((2, 4), (6, 8)),
                                    ((2, 3), (4, 5)),  
                                    ((5, 7), (7, 9)), 
                                    ((2, 8), (3, 7)), 
                                    ((6, 6), (4, 6)), 
                                    ((2, 6), (4, 8))]);
}
#[test]
fn test_day04_part1() {
    assert_eq!(part1(&prep(DAY4_EXAMPLE)), 2);
}

#[test]
fn test_day04_part2() {
    assert_eq!(part2(&prep(DAY4_EXAMPLE)), 4);
}
