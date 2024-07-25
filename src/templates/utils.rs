use std::fs::{read_to_string, File};
use std::io::{self, BufRead};

pub fn file_to_string(year: &str, day: &str) -> String {
    let path = format!("aoc/src/aoc_{year}/inputs/input_{day}.txt");
    read_to_string(format!("src/aoc_{year}/inputs/input_{day}.txt")).unwrap()
}

pub fn file_to_lines(year: &str, day: &str) -> Vec<String> {
    let path = format!("aoc/src/aoc_{year}/inputs/input_{day}.txt");
    let file = File::open(path).expect("No such file.");
    let buf = io::BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Error parsing line"))
        .collect()
}
