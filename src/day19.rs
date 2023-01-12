use crate::check_or_get_input;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, time::Instant};

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
#[allow(non_camel_case_types)]
type cache_t = HashMap<([u32; 4], [u32; 4], u32), u32>;

fn search(
    blueprint: &Blueprint,
    time_left: u32,
    materials: [u32; 4],
    robots: [u32; 4],
    cache: &mut cache_t,
) -> u32 {
    if time_left == 0 {
        cache.insert((materials, robots, 0), materials[3]);
        return materials[3];
    }
    if let Some(r) = cache.get(&(materials, robots, time_left)) {
        return r.to_owned();
    }
    let mut max_geodes = search(
        &blueprint,
        time_left - 1,
        [
            materials[0] + robots[0],
            materials[1] + robots[1],
            materials[2] + robots[2],
            materials[3] + robots[3],
        ],
        [robots[0], robots[1], robots[2], robots[3]],
        cache,
    );
    for i in 0..4 {
        if i != 3
            && robots[i] >= blueprint.costs[0][i]
            && robots[i] >= blueprint.costs[1][i]
            && robots[i] >= blueprint.costs[2][i]
            && robots[i] >= blueprint.costs[3][i]
        {
            continue;
        }

        let mut collect_time = 0;
        for j in 0..3 {
            if materials[j] >= blueprint.costs[i][j] {
                //No need to wait for this raw material
                continue;
            }
            if blueprint.costs[i][j] > 0 && robots[j] == 0 {
                // No way to build robot_i right now
                collect_time = 999999; //Simulate a infinite calendar
                break;
            }
            let t = if (blueprint.costs[i][j] - materials[j]) % robots[j] == 0 {
                (blueprint.costs[i][j] - materials[j]) / robots[j]
            } else {
                (blueprint.costs[i][j] - materials[j]) / robots[j] + 1
            };
            collect_time = std::cmp::max(collect_time, t);
        }
        collect_time += 1; //we need one more round to build robot_i

        if collect_time > time_left {
            //No way to do this in time
            continue;
        }
        let mut new_robots = robots;
        let mut new_materials = materials;
        for j in 0..4 {
            new_materials[j] += robots[j] * collect_time;
            if j != 3 {
                new_materials[j] -= blueprint.costs[i][j];
            }
        }
        new_robots[i] += 1;
        let geodes = search(
            &blueprint,
            time_left - collect_time,
            new_materials,
            new_robots,
            cache,
        );
        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }
    cache.insert((materials, robots, time_left), max_geodes);
    return max_geodes;
}

fn prep(input: &str) -> Vec<Blueprint> {
    parse(input)
}

fn part1(blueprints: &Vec<Blueprint>) -> u32 {
    let result = blueprints
        .into_par_iter()
        .map(|x| {
            let mut cache = cache_t::new();
            let maxg = search(&x, 24, [0, 0, 0, 0], [1, 0, 0, 0], &mut cache);
            println!("{}: {}", x.id, maxg);
            x.id * maxg
        })
        .sum();
    result
}

fn part2(blueprints: &Vec<Blueprint>) -> u32 {
    let result = blueprints
        //Doing all three blueprints in parallel uses a lot of memory (~12GB)
        //It might be better run these sequentially for low-memory machines.
        .into_par_iter() //This should be .iter() for sequential operation, .into_par_iter() for parallel
        .take(3)
        .map(|x| {
            let mut cache = cache_t::new();
            let maxg = search(&x, 32, [0, 0, 0, 0], [1, 0, 0, 0], &mut cache);
            println!("{}: {}", x.id, maxg);

            maxg * x.id
        })
        .product();
    result
}

pub fn solve() {
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
