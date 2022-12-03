
use std::{fs::File, io::{Write, self, BufRead}, path::Path};
use reqwest::{header::COOKIE};
use dotenv;

#[derive(Debug)]
pub enum AOCError {
    IO(std::io::Error),
    Config(dotenv::Error),
    Net(reqwest::Error)
}

impl From<std::io::Error> for AOCError {
    fn from(e: std::io::Error) -> Self {
        AOCError::IO(e)
    }
}

impl From<dotenv::Error> for AOCError {
    fn from(e: dotenv::Error) -> Self {
        AOCError::Config(e)
    }
}

impl From<reqwest::Error> for AOCError {
    fn from(e: reqwest::Error) -> Self {
        AOCError::Net(e)
    }
}

fn  get_input(day: usize, session_id: &String) -> Result<bytes::Bytes, AOCError> {
//    println!("{session_id}");
   let endpoint = format!("https://adventofcode.com/2022/day/{day}/input");
//    println!("{endpoint}");

   let client = reqwest::blocking::Client::new();
   let content = client
    .get(endpoint)
    .header(COOKIE, format!("session={session_id}"))
    .send()?.bytes()?;

    Ok(content)
}

fn save_input(day: usize, filepath: &str, session_id: Option<String>) -> Result<(), AOCError> {
    let session_id = match session_id {
        Some(s) => s,
        None => dotenv::var("AOC_SESSION_ID")? 
    };
   let content = get_input(day, &session_id)?;
   let mut f = File::create(filepath)?;
   let written = f.write(&content)?;
   if written < content.len() {
    return Err(AOCError::IO(std::io::Error::new(std::io::ErrorKind::Interrupted, "File was not completely written!")));  
   }
   return Ok(());
}

pub fn check_or_get_input(day: usize) -> Result<String, AOCError>  {
    let filepath = format!("./inputs/{day:0>2}.txt");
    // Does the input file already exist?
    if !Path::new(&filepath).exists() {
        save_input(day, filepath.as_str(), None)?;
    }
    return Ok(filepath)
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


