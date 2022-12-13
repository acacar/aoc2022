use dotenv;
use reqwest::header::COOKIE;
use std::{
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;

fn get_input(day: usize, session_id: &String) -> Result<bytes::Bytes, reqwest::Error> {
    let endpoint = format!("https://adventofcode.com/2022/day/{day}/input");
    let client = reqwest::blocking::Client::new();
    let content = client
        .get(endpoint)
        .header(COOKIE, format!("session={session_id}"))
        .send()?
        .bytes()?;
    Ok(content)
}

fn save_input(
    day: usize,
    filepath: &str,
    session_id: Option<String>,
) -> Result<(), std::io::Error> {
    let session_id = match session_id {
        Some(s) => s,
        None => dotenv::var("AOC_SESSION_ID").expect("Session ID not defined."),
    };
    let content =
        get_input(day, &session_id).expect(format!("Cannot get input for day {day}").as_str());
    let mut f = File::create(filepath)?;
    f.write(&content)?;
    return Ok(());
}

pub fn check_or_get_input(day: usize) -> String {
    let filepath = format!("./inputs/{day:0>2}.txt");
    // Does the input file already exist?
    if !Path::new(&filepath).exists() {
        save_input(day, filepath.as_str(), None)
            .expect(format!("Could not save input for day {day}").as_str());
    }
    filepath
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
