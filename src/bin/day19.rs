use aoc2022::check_or_get_input;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
struct Blueprint {
    id: u32,
    costs: [[u32; 3]; 4],
}

fn parse(input: &str) -> Vec<Blueprint> {
    let mut blueprints: Vec<Blueprint> = vec![];
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian")
        .unwrap();
    for line in input.lines() {
        let captures: Vec<u32> = re
            .captures(line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        blueprints.push(Blueprint {
            id: captures[0],
            costs: [
                [captures[1], 0, 0],
                [captures[2], 0, 0],
                [captures[3], captures[4], 0],
                [captures[5], 0, captures[6]],
            ],
        });
    }
    blueprints
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    time_left: u32,
    materials: [u32; 4],
    robots: [u32; 4],
}

impl State {
    fn mine(&self, duration: u32) -> Option<State> {
        let mut newstate = self.clone();
        for i in 0..4 {
            newstate.materials[i] += newstate.robots[i];
        }
        if (newstate.time_left as i32 - duration as i32) < 0 {
            None
        } else {
            newstate.time_left -= duration;
            Some(newstate)
        }
    }

    fn make_robot(&self, blueprint: &Blueprint, robot_type: usize) -> Option<State> {
        for material_needed in 0..3 {
            if self.materials[material_needed] < blueprint.costs[robot_type][material_needed] {
                // println!("{:?} < {:?}", self.materials, blueprint.costs[robot_type]);
                return None;
            }
        }
        if let Some(mut s) = self.mine(1) {
            for cost_idx in 0..3 {
                s.materials[cost_idx] -= blueprint.costs[robot_type][cost_idx]
            }
            s.robots[robot_type] += 1;

            return Some(s);
        }
        None
    }
}

fn is_dominated(current: State, best: &State) -> bool {
    return (current.materials[3]
        + current.robots[3] * current.time_left
        + ((current.time_left) * (current.time_left + 1) / 2))
        <= best.materials[3];
}

fn fanout(prod_rate_limit: [u32; 3], blueprint: &Blueprint, parent: &State) -> Vec<State> {
    let mut children = vec![];
    if let Some(s) = parent.mine(1) {
        children.push(s);
    }

    //Prioritize branches that make later robots
    for robot_type in [3, 2, 1, 0].iter() {
        //Can we benefit by making more of this robot
        if *robot_type == 3 || parent.robots[*robot_type] < prod_rate_limit[*robot_type] {
            if let Some(s) = parent.make_robot(blueprint, *robot_type) {
                children.push(s);
            }
        }
    }
    children
}

fn search(blueprint: &Blueprint, time: u32) -> u32 {
    let initial = State {
        time_left: time,
        materials: [0, 0, 0, 0],
        robots: [1, 0, 0, 0],
    };
    let mut visited: HashSet<State> = HashSet::new();
    let mut stack: Vec<State> = Vec::new();
    let mut best: State = initial;

    let mut prod_rate_limit: [u32; 3] = [0, 0, 0];
    for i in 0..3 {
        let mut max_cost = 0;
        for j in 0..4 {
            max_cost = std::cmp::max(max_cost, blueprint.costs[j][i]);
        }
        prod_rate_limit[i] = max_cost;
    }
    stack.push(initial);

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        if current.materials[3] > best.materials[3] {
            best = current;
        }
        if current.time_left > 0 {
            if !is_dominated(current, &best) {
                if !visited.contains(&current) {
                    visited.insert(current);
                    stack.extend(fanout(prod_rate_limit, blueprint, &current).iter());
                }
            }
        }
    }
    best.materials[3]
}

fn prep(input: &str) -> Vec<Blueprint> {
    parse(input)
}

fn part1(blueprints: &Vec<Blueprint>) -> u32 {
    let result = blueprints
        .into_par_iter()
        .map(|x| {
            let maxg = search(&x, 24);
            x.id * maxg
        })
        .sum();
    result
}

fn part2(blueprints: &Vec<Blueprint>) -> u32 {
    //Doing all three blueprints in parallel may use a lot of memory
    //It might be better run these sequentially for low-memory machines.
    blueprints
        .into_par_iter() //This should be .iter() for sequential operation, .into_par_iter() for parallel
        .take(3)
        .map(|x| {
            let maxg = search(&x, 32);
            maxg
        })
        .product()
}

pub fn main() {
    let filename: String = check_or_get_input(19);
    let blueprints = prep(
        std::fs::read_to_string(filename)
            .expect("Day 19: cannot read input")
            .as_str(),
    );
    let mut st = Instant::now();
    let tmp = part1(&blueprints);
    println!(
        "Day 19, part1: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
    st = Instant::now();
    let tmp = part2(&blueprints);
    println!(
        "Day 19, part2: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
}

#[test]
fn test_day19_part1() {
    let blueprints = prep(
        std::fs::read_to_string(check_or_get_input(19))
            .expect("Day 19: cannot read input")
            .as_str(),
    );
    println!("{:?}", part1(&blueprints));
}

#[test]
fn test_day19_part2() {}

#[allow(dead_code)]
const DAY19_EXAMPLE: &str = r#""#;
