use crate::check_or_get_input;

fn prep(input: String) -> Vec<i32> {
    let mut data: Vec<i32> = vec![];
    let mut subtotal = 0;
    for item in input.lines() {
        if item == "" {
            data.push(subtotal);
            subtotal = 0;
        } else {
            subtotal += item
                .parse::<i32>()
                .expect(format!("Bad input line: {}", item).as_str());
        }
    }
    if subtotal > 0 {
        data.push(subtotal)
    }
    data.sort_by(|a, b| b.cmp(a));
    return data;
}

fn part1(data: &Vec<i32>) -> i32 {
    return data[0];
}

fn part2(data: &Vec<i32>) -> i32 {
    return data[0..3].iter().sum();
}

pub fn solve() {
    let filename: String = check_or_get_input(1);
    let data = prep(std::fs::read_to_string(filename).expect("Day 1: cannot read input"));
    println!("Day 1, part 1: {}", part1(&data));
    println!("Day 1, part 2: {}", part2(&data));
}

// Tests

#[allow(dead_code)]
const DAY1_EXAMPLE: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

#[test]
fn test_day01_prep() {
    assert_eq!(
        prep(String::from(DAY1_EXAMPLE)),
        vec![24000, 11000, 10000, 6000, 4000]
    );
}

#[test]
fn test_day01_part1() {
    let input = DAY1_EXAMPLE;
    assert_eq!(part1(&prep(String::from(input))), 24000);
}

#[test]
fn test_day01_part2() {
    assert_eq!(part2(&prep(String::from(DAY1_EXAMPLE))), 45000);
}
