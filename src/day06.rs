use crate::check_or_get_input;

fn prep(input: &str) -> String {
    input.to_string()
}

fn has_duplicates(window: &str) -> bool {
    let mut chars: Vec<char> = window.chars().collect();
    chars.sort();
    for i in 0..(chars.len() - 1) {
        if chars[i] == chars[i + 1] {
            return true;
        }
    }
    false
}

fn part1(foo: &String) -> i32 {
    let mut i = 0;
    while i < foo.len() - 4 {
        let window = &foo[i..(i + 4)];
        if has_duplicates(window) {
            i += 1;
        } else {
            break;
        };
    }
    (i as i32) + 4
}

fn part2(foo: &String) -> i32 {
    let mut i = 0;
    while i < foo.len() - 14 {
        let window = &foo[i..(i + 14)];
        if has_duplicates(window) {
            i += 1;
        } else {
            break;
        };
    }
    (i as i32) + 14
}

pub fn solve() {
    let filename: String = check_or_get_input(6);
    let data = prep(
        std::fs::read_to_string(filename)
            .expect("Day 6: cannot read input")
            .as_str(),
    );
    println!("Day 06, part1: {}", part1(&data));
    println!("Day 06, part2: {}", part2(&data));
}

// TESTS

#[allow(dead_code)]
const DAY06_EXAMPLES_P1: (&str, &str, &str, &str) = (
    "bvwbjplbgvbhsrlpgdmjqwftvncz",
    "nppdvjthqldpwncqszvftbrmjlhg",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
);

#[allow(dead_code)]
const DAY06_EXAMPLES_P2: (&str, &str, &str, &str, &str) = (
    "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
    "bvwbjplbgvbhsrlpgdmjqwftvncz",
    "nppdvjthqldpwncqszvftbrmjlhg",
    "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
    "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
);

#[allow(dead_code)]
const DAY06_EXPECTED_PART1: (i32, i32, i32, i32) = (5, 6, 10, 11);
#[allow(dead_code)]
const DAY06_EXPECTED_PART2: (i32, i32, i32, i32, i32) = (19, 23, 23, 29, 26);

#[test]
fn test_day06_prep() {}
#[test]
fn test_day06_part1() {
    for _ in 0..4 {
        assert_eq!(
            DAY06_EXPECTED_PART1.0,
            part1(&DAY06_EXAMPLES_P1.0.to_string())
        );
    }
}

#[test]
fn test_day06_part2() {
    for _ in 0..4 {
        assert_eq!(
            DAY06_EXPECTED_PART2.0,
            part2(&DAY06_EXAMPLES_P2.0.to_string())
        );
    }
}
