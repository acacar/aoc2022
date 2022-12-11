use std::{collections::HashMap, time::Instant};

use crate::check_or_get_input;

type Trace = HashMap<i32, i32>;

#[allow(dead_code)]
fn trace_fmt(trace: &Trace) -> String {
    let mut retstr = "Trace:\n".to_string();
    let mut keys: Vec<i32> = trace.keys().map(|x| x.to_owned()).collect();
    keys.sort();
    for key in keys {
        retstr = retstr + &format!("{} : {}\n", key, trace.get(&key).unwrap()).to_string();
    }
    retstr
}

#[derive(Debug)]
enum Instruction {
    Nop,
    Addx(i32),
}

fn prep(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    for line in input.lines() {
        let mut line = line.split(" ");
        let newins = match line.next().unwrap() {
            "noop" => Instruction::Nop,
            "addx" => Instruction::Addx(line.next().unwrap().parse().unwrap()),
            _ => panic!("Unknown instruction"),
        };
        result.push(newins)
    }
    result
}

fn make_trace(instructions: &Vec<Instruction>) -> Trace {
    let mut trace: Trace = HashMap::new();
    let mut x: i32 = 1;
    let mut pc: i32 = 1;
    for instruction in instructions {
        match instruction {
            Instruction::Nop => {
                trace.insert(pc, x);
                pc += 1
            }
            Instruction::Addx(a) => {
                trace.insert(pc, x);
                pc += 1;
                trace.insert(pc, x);
                pc += 1;
                x += a;
            }
        }
    }
    trace
}

fn part1(input: &Vec<Instruction>) -> i32 {
    let trace = make_trace(input);
    let mut total = 0;
    // println!("{}", trace_fmt(&trace));
    for i in vec![20, 60, 100, 140, 180, 220] {
        total += i * trace.get(&i).unwrap();
    }
    total
}

fn is_lit(pixel: i32, trace: &Trace) -> bool {
    let cycle = pixel + 1;
    let mid_sprite = trace.get(&cycle).unwrap().to_owned();
    let sprite = (mid_sprite - 1, mid_sprite, mid_sprite + 1);
    let rowpos = pixel % 40;
    //return ((rowpos != 0) && (rowpos  == sprite.0)) || rowpos == sprite.1 || ((rowpos != 39) && (rowpos == sprite.2))
    return rowpos == sprite.0 || rowpos == sprite.1 || rowpos == sprite.2;
    // return ( pixel % 40 != 0 && trace.get(&cycle).unwrap().to_owned() == row_pos )
    //     || trace.get(&cycle).unwrap().to_owned() == row_pos
    //     || ( pixel % 40 != 39 && trace.get(&cycle).unwrap().to_owned() == row_pos);
}

fn part2(instructions: &Vec<Instruction>) -> String {
    let trace = make_trace(instructions);
    // println!("{}", trace_fmt(&trace));
    let mut retstr = "\n".to_string();
    for row in 0..6 {
        let mut scanline = "".to_string();
        for pixel in 0..40 {
            if is_lit(row * 40 + pixel, &trace) {
                scanline.push('#');
            } else {
                scanline.push('.');
            }
        }
        scanline.push('\n');
        retstr = retstr + &scanline;
    }
    retstr
}

pub fn solve() {
    let filename: String = check_or_get_input(10);
    let instructions = prep(
        std::fs::read_to_string(filename)
            .expect("Day 10: cannot read input")
            .as_str(),
    );
    let mut st = Instant::now();
    let tmp = part1(&instructions);
    println!(
        "Day 10, part1: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
    st = Instant::now();
    let tmp = part2(&instructions);
    println!(
        "Day 10, part2:\n {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );

}

// TESTS

#[test]
fn test_day10_prep() {
    assert_eq!(
        "[Nop, Addx(3), Addx(-5)]",
        format!("{:?}", prep(DAY10_EXAMPLE_1))
    );
}
#[test]
fn test_day10_part1() {
    assert_eq!(DAY10_EXPECTED_2_1, part1(&prep(DAY10_EXAMPLE_2)))
}

#[test]
fn test_day10_part2() {
    assert_eq!(DAY10_EXPECTED_2_2, part2(&prep(DAY10_EXAMPLE_2)))
}

#[allow(dead_code)]
const DAY10_EXAMPLE_1: &str = r#"noop
addx 3
addx -5"#;

#[allow(dead_code)]
const DAY10_EXPECTED_2_1: i32 = 13140;

#[allow(dead_code)]
const DAY10_EXPECTED_2_2: &str = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#;

#[allow(dead_code)]
const DAY10_EXAMPLE_2: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
