use aoc2022::check_or_get_input;
use std::time::Instant;

fn prep(input: &str) -> Vec<i64> {
    let mut ciphertext: Vec<i64> = vec![];
    for line in input.lines() {
        ciphertext.push(line.parse().unwrap());
    }
    ciphertext
}

fn mix(ciphertext: &Vec<i64>, repeat: usize) -> Vec<i64> {
    let n = ciphertext.len();
    let mut mix: Vec<usize> = (0..n).collect();
    for _ in 0..repeat {
        for i in 0..n {
            let idx_src = mix.iter().position(|x| x == &i).unwrap();
            let idx_to_dst = ((idx_src as i64) + ciphertext[i]).rem_euclid((n - 1) as i64) as usize;
            mix.remove(idx_src);
            mix.insert(idx_to_dst, i)
        }
    }
    let mut result = vec![0; ciphertext.len()];
    for (i, v) in mix.iter().enumerate() {
        result[i] = ciphertext[v.to_owned()];
    }
    result
}

fn part1(ciphertext: &Vec<i64>) -> i64 {
    let plaintext = mix(ciphertext, 1);
    let root_pos = plaintext.iter().position(|v| v.to_owned() == 0i64).unwrap();
    let m = ciphertext.len();
    plaintext[(1000 + root_pos) % m]
        + plaintext[(2000 + root_pos) % m]
        + plaintext[(3000 + root_pos) % m]
}

fn part2(ciphertext: &Vec<i64>) -> i64 {
    let keyed_ciphertext: Vec<i64> = ciphertext.iter().map(|x| x * 811589153).collect();
    let plaintext = mix(&keyed_ciphertext, 10);
    let root_pos = plaintext.iter().position(|v| v.to_owned() == 0i64).unwrap();
    let m = ciphertext.len();
    plaintext[(1000 + root_pos) % m]
        + plaintext[(2000 + root_pos) % m]
        + plaintext[(3000 + root_pos) % m]
}

pub fn main() {
    let filename: String = check_or_get_input(20);
    let ciphertext = prep(
        std::fs::read_to_string(filename)
            .expect("Day 20: cannot read input")
            .as_str(),
    );
    let mut st = Instant::now();
    let tmp = part1(&ciphertext);
    println!(
        "Day 20, part1: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
    st = Instant::now();
    let tmp = part2(&ciphertext);
    println!(
        "Day 20, part2: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
}

#[test]
fn test_day20_part1() {
    println!("{:?}", part1(&prep(DAY20_EXAMPLE)))
}

#[test]
fn test_day20_part2() {
    println!("{:?}", part2(&prep(DAY20_EXAMPLE)))
}

#[allow(dead_code)]
const DAY20_EXAMPLE: &str = r#"1
2
-3
3
-2
0
4"#;
