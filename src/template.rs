use std::{fs};
use std::error::Error;
use std::io::ErrorKind;
use std::path::Path;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};

const TEMPLATE: &str = include_str!("aoc_template.txt");

pub fn new_day(year: &str, day: &str) {
    let base_path = format!("src/aoc_{year}");
    if !Path::new(&base_path).exists() {
        eprintln!("Please cd to the root of your aoc directory");
        return;
    }

    let session;
    if let Ok(s) = fs::read_to_string("session.txt") {
        session = s;
    } else {
        eprintln!("Missing session.txt in project root to authenticate with AoC");
        return;
    }

    if let Err(e) = write_solution_template(&base_path, year, day) {
        eprintln!("Error writing solution template: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = write_solution_mod(&base_path, day) {
        eprintln!("Error updating mod.rs: {}", e);
        std::process::exit(1);
    }
    if let Err(e) = write_input(&base_path, year, day, &session) {
        eprintln!("Error writing input file: {}", e);
        std::process::exit(1);
    }
}

pub fn new_year(year: &str) {
    // cd src
    // create_dir aoc_{year}
    // create utils
    // cd aoc_{year}
    // create mod
    // create_dir solutions
    // cd solutions
    // create mod
    println!("Triggered submodule creation for {year}");
}

fn write_solution_template(base_path: &str, year: &str, day: &str) -> Result<(), Box<dyn Error>> {
    let content = TEMPLATE.replace("{{year}}", year).replace("{{day}}", day);
    let filename = format!("day_{day}.rs");
    let file_path = Path::new(&base_path).join("solutions").join(&filename);
    write_file(&file_path, &content)?;
    println!("<Created new file {filename} for AoC {year}>");
    Ok(())
}

fn write_solution_mod(base_path: &str, day: &str) -> Result<(), Box<dyn Error>> {
    let mod_path = Path::new(&base_path).join("solutions").join("mod.rs");
    let mut content = if mod_path.exists() {
        fs::read_to_string(&mod_path)?
    } else {
        String::new()
    };
    let add_to_mod = format!("pub mod day_{};\n", day);
    if !content.contains(&add_to_mod) {
        content.push_str(&add_to_mod);
        fs::write(mod_path, content)?;
        println!("<Updated mod.rs>");
    }

    Ok(())
}

fn write_input(
    base_path: &str,
    year: &str,
    day: &str,
    session: &str,
) -> Result<(), Box<dyn Error>> {
    let filename = format!("input_{day}.txt");
    let input_dir = Path::new(&base_path).join("inputs");
    if !input_dir.exists() {
        fs::create_dir(input_dir)?;
    }
    let file_path = Path::new(&base_path).join("inputs").join(&filename);
    let d = day.parse::<i32>().unwrap();
    let url = format!("https://adventofcode.com/{year}/day/{d}/input");
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?;
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(session)?);
    let response = client.get(url).headers(headers).send()?;
    write_file(&file_path, &response.text()?)?;
    println!("<Retrieved puzzle input>");
    Ok(())
}

fn write_file(full_path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    if !full_path.exists() {
        fs::write(full_path, content)?;
    } else {
        let err = std::io::Error::new(ErrorKind::AlreadyExists, "File already exists");
        return Err(Box::new(err));
    }
    Ok(())
}
