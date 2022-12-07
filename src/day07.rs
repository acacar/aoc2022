use std::collections::HashMap;

use sscanf::sscanf;
use uuid::Uuid;

use crate::check_or_get_input;

#[derive(Debug)]
struct Directory {
    id: Uuid,
    parent: Option<Uuid>,
    children: HashMap<String, Uuid>,
    file_total: i32,
}

struct MasterIndex {
    directories: HashMap<Uuid, Directory>,
}

impl Directory {
    fn new(parent: Option<Uuid>) -> Directory {
        Directory {
            id: Uuid::new_v4(),
            parent,
            children: HashMap::new(),
            file_total: 0,
        }
    }

    fn add_file(&mut self, size: i32) {
        self.file_total += size;
    }
}

fn prep(input: &str) -> (MasterIndex, Uuid) {
    // Assuming first line is the root directory, i.e. (cd /)
    let listing: Vec<String> = input
        .to_string()
        .lines()
        .map(|x| x.to_string())
        .skip(1)
        .collect();
    let mut index = MasterIndex {
        directories: HashMap::new(),
    };
    let root = Directory::new(None);
    let root_id = root.id.clone();
    let mut current_id = root_id.clone();
    index.directories.insert(root.id.clone(), root);

    for line in listing.iter() {
        if line.chars().nth(0).unwrap() == '$' {
            // This is a command, not a directory listing
            match sscanf!(line, "$ {} {}", str, str) {
                Ok(("cd", "..")) => {
                    current_id = match index.directories.get(&current_id).unwrap().parent {
                        Some(id) => id,
                        None => panic!("Cannot go up from root directory"),
                    }
                    .clone();
                }
                Ok(("cd", dir)) => {
                    current_id = match index
                        .directories
                        .get(&current_id)
                        .unwrap()
                        .children
                        .get(dir)
                    {
                        Some(id) => id.clone(),
                        None => panic!("Cannot find directory {}", dir),
                    };
                }
                Ok((a, b)) => {
                    panic!("Unknown command: {} {}", a, b);
                }
                Err(_) => {
                    // Must have been an "ls"
                }
            }
        } else {
            // This is a directory listing
            let mut parts = line.split_whitespace();
            let part1 = parts.next().unwrap().to_string();
            let part2 = parts.next().unwrap().to_string();
            if part1.chars().nth(0).unwrap() == 'd' {
                let name = part2;
                // This is a directory
                let dir = Directory::new(Some(current_id.clone()));
                index
                    .directories
                    .get_mut(&current_id)
                    .unwrap()
                    .children
                    .insert(name, dir.id.clone());
                index.directories.insert(dir.id.clone(), dir);
            } else {
                // This is a file
                let size: i32 = part1.parse().unwrap();
                index
                    .directories
                    .get_mut(&current_id)
                    .unwrap()
                    .add_file(size);
            }
        }
    }
    (index, root_id)
}

fn dir_size(root: Uuid, index: &MasterIndex) -> i32 {
    let current_id = root.clone();
    let current_dir = index.directories.get(&current_id).unwrap();
    let mut total = current_dir.file_total;
    for child_id in current_dir.children.values() {
        total += dir_size(child_id.clone(), index);
    }
    total
}

fn part1(index: &MasterIndex) -> i32 {
    //not very efficient, but what the hell...
    //TODO: maybe add memoization later
    let mut grand_total = 0;
    for dir in index.directories.values() {
        let dir_total = dir_size(dir.id.clone(), index);
        if dir_total <= 100000 {
            grand_total += dir_total;
        }
    }
    grand_total
}

fn part2(root_id: Uuid, index: &MasterIndex) -> Option<i32> {
    let total_space = 70_000_000;
    let required_space = 30_000_000;
    let max_full = total_space - required_space;
    let used = dir_size(root_id.clone(), index);
    let min_delete = used - max_full;
    let mut dir_sizes = vec![];
    for dir in index.directories.values() {
        dir_sizes.push(dir_size(dir.id.clone(), index));
    }
    dir_sizes.sort();
    for ds in dir_sizes {
        if ds >= min_delete {
            return Some(ds);
        }
    }
    None
}

pub fn solve() {
    let filename: String = check_or_get_input(7);
    let (index, root_id) = prep(
        std::fs::read_to_string(filename)
            .expect("Day 7: cannot read input")
            .as_str(),
    );
    println!("Day 07, part1: {}", part1(&index));
    println!(
        "Day 07, part2: {}",
        part2(root_id.clone(), &index).expect("No solution found")
    );
}

// TESTS

#[allow(dead_code)]
const DAY07_EXAMPLE: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

#[test]
fn test_day06_prep() {
    let (index, root_id) = prep(DAY07_EXAMPLE);
    assert_eq!(index.directories.len(), 4);
    assert_eq!(dir_size(root_id, &index), 48_381_165);
}
#[test]
fn test_day06_part1() {
    let (index, _) = prep(DAY07_EXAMPLE);
    assert_eq!(part1(&index), 95_437);
}

#[test]
fn test_day06_part2() {
    let (index, root_id) = prep(DAY07_EXAMPLE);
    assert_eq!(
        part2(root_id.clone(), &index).expect("No solution found"),
        24_933_642
    );
}
