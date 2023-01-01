use crate::check_or_get_input;
use regex::Regex;
use std::{collections::HashMap, time::Instant};

fn parse(input: &str) -> Vec<(String, i32, Vec<String>)> {
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnel.? lead.? to valve.? (.+)")
        .unwrap();
    let mut nodes = vec![];
    for line in input.lines() {
        let captures = re.captures(line).unwrap();
        let mut placeholders = captures.iter();
        placeholders.next();
        let name = placeholders.next().unwrap().unwrap().as_str().to_owned();
        let flow: i32 = placeholders
            .next()
            .unwrap()
            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        let neighbors: Vec<String> = placeholders
            .next()
            .unwrap()
            .unwrap()
            .as_str()
            .to_owned()
            .split(", ")
            .map(|i| i.to_owned())
            .collect();
        nodes.push((name, flow, neighbors));
    }
    nodes
}

#[derive(Debug)]
struct CaveSystem {
    nodes: HashMap<String, usize>,
    tunnels: Vec<Vec<i32>>,
    flows: Vec<i32>,
}

fn prep(input: &str) -> CaveSystem {
    let node_tuples = parse(input);
    let caves = create_caves(node_tuples);
    let caves = compress_caves(caves);
    //println!("{:?}", caves);
    caves
}

fn create_caves(node_tuples: Vec<(String, i32, Vec<String>)>) -> CaveSystem {
    let mut node_names = node_tuples
        .iter()
        .map(|(name, _, _)| name)
        .collect::<Vec<_>>();
    node_names.sort();
    let mut nodes = HashMap::new();
    node_names.iter().enumerate().for_each(|(i, name)| {
        nodes.insert(name.to_string(), i);
    });
    let mut tunnels = vec![vec![9999; nodes.len()]; nodes.len()];
    let mut flows = vec![0; nodes.len()];
    for (name, flow, neighbors) in node_tuples.clone() {
        let i = nodes[&name];
        flows[i] = flow;
        for neighbor in neighbors {
            let j = nodes[&neighbor];
            tunnels[i][j] = 1;
        }
    }
    for i in 0..nodes.len() {
        tunnels[i][i] = 0;
    }
    // use the floyd-warshall algorithm to find the shortest path between all nodes
    for k in 0..nodes.len() {
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if tunnels[i][j] > tunnels[i][k] + tunnels[k][j] {
                    tunnels[i][j] = tunnels[i][k] + tunnels[k][j];
                }
            }
        }
    }
    CaveSystem {
        nodes,
        tunnels,
        flows,
    }
}

fn compress_caves(c: CaveSystem) -> CaveSystem {
    let mut cmp_nodes = HashMap::new();
    let mut nonzero_nodes: Vec<String> = c
        .nodes
        .iter()
        .filter(|(name, &i)| name.to_string() == "AA" || c.flows[i] > 0)
        .map(|(name, _)| name.to_string())
        .collect();
    nonzero_nodes.sort();
    nonzero_nodes.iter().enumerate().for_each(|(i, name)| {
        cmp_nodes.insert(name.to_string(), i);
    });
    let mut cmp_tunnels = vec![vec![9999; nonzero_nodes.len()]; nonzero_nodes.len()];
    for (src, src_idx) in cmp_nodes.iter() {
        for (dst, dst_idx) in cmp_nodes.iter() {
            cmp_tunnels[*src_idx][*dst_idx] = c.tunnels[c.nodes[src]][c.nodes[dst]];
        }
    }
    let mut cmp_flows = vec![0; nonzero_nodes.len()];
    for (name, idx) in cmp_nodes.iter() {
        cmp_flows[*idx] = c.flows[c.nodes[name]];
    }
    //println!("{:?}", cmp_nodes);
    CaveSystem {
        nodes: cmp_nodes,
        tunnels: cmp_tunnels,
        flows: cmp_flows,
    }
}

type Cache = HashMap<(i32, usize, usize), i32>;

fn search(time: i32, node: usize, visited: usize, cache: &mut Cache, caves: &CaveSystem) -> i32 {
    match cache.get(&(time, node, visited)) {
        Some(x) => return x.to_owned(),
        None => (),
    }
    let mut max_flow = 0;
    for n in 0..caves.nodes.len() {
        if n == node {
            continue;
        }
        let n_mask = 1 << n;
        if visited & n_mask != 0 {
            continue;
        }
        let time_remaining = time - caves.tunnels[node][n] - 1;
        if time_remaining <= 0 {
            continue;
        }
        let n_flow = search(time_remaining, n, visited | n_mask, cache, caves)
            + caves.flows[n] * time_remaining;
        if n_flow > max_flow {
            max_flow = n_flow;
        }
    }
    cache.insert((time, node, visited), max_flow);
    max_flow
}

fn part1(caves: &CaveSystem) -> i32 {
    let mut cache = Cache::new();
    let retval = search(30, 0, 0, &mut cache, &caves);
    retval
}

#[allow(unused_variables)]
fn part2(caves: &CaveSystem) -> i32 {
    let mut cache = Cache::new();

    let fullmask = (2 as usize).pow(caves.nodes.len() as u32) - 1;
    let mut max_flow = 0;
    for partition in 0..(fullmask + 1) {
        let p1 = search(26, 0, partition, &mut cache, &caves);
        let p2 = search(26, 0, fullmask ^ partition, &mut cache, &caves);
        if p1 + p2 > max_flow {
            max_flow = p1 + p2;
        }
    }
    max_flow
}

pub fn solve() {
    let filename: String = check_or_get_input(16);
    let caves = prep(
        std::fs::read_to_string(filename)
            .expect("Day 16: cannot read input")
            .as_str(),
    );
    let mut st = Instant::now();
    let tmp = part1(&caves);
    println!(
        "Day 16, part1: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
    st = Instant::now();
    let tmp = part2(&caves);
    println!(
        "Day 16, part2: {} ({} us)",
        tmp,
        (Instant::now() - st).as_micros()
    );
}

#[test]
fn test_day16_part1() {
    let caves = prep(DAY16_EXAMPLE);
    assert_eq!(1651, part1(&caves));
}

#[test]
fn test_day16_part2() {
    let caves = prep(DAY16_EXAMPLE);
    assert_eq!(1707, part2(&caves));
}

#[allow(dead_code)]
const DAY16_EXAMPLE: &str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"#;
