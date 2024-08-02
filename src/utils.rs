use crate::Config;
use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use regex::Regex;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};
use crate::config::AdventUnit;

pub fn read_config() -> Option<Config> {
    if let Ok(content) = fs::read_to_string("elf.toml") {
        let config = toml::from_str(&content).expect("Error parsing elf.toml");
        Some(config)
    } else {
        eprintln!("Couldn't find elf.toml. Are you in the correct project root?");
        None
    }
}

pub fn write_new_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    if !path.exists() {
        fs::write(path, content)?;
    } else {
        let err = std::io::Error::new(ErrorKind::AlreadyExists, "File already exists");
        return Err(Box::new(err));
    }
    Ok(())
}

pub fn write_to_file(path: &Path, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file_content = if path.exists() {
        fs::read_to_string(&path)?
    } else {
        String::new()
    };
    file_content.push_str(content);
    fs::write(path, file_content)?;
    Ok(())
}

pub fn update_elf(
    year: Option<String>,
    day: Option<String>,
    session: Option<String>,
    template: Option<String>,
    cfg: &mut Config,
) -> Result<(), Box<dyn Error>> {
    year.map(|y| cfg.year = y);
    day.map(|d| cfg.day = d);
    session.map(|s| cfg.session = s);
    template.map(|t| cfg.template = Some(t));
    let elf = toml::ser::to_string(&cfg)?;
    fs::write("elf.toml", elf)?;
    Ok(())
}

pub fn load_input(year: &str, day: &str, session: &str) -> Result<String, Box<dyn Error>> {
    let d = day.parse::<i32>().unwrap();
    let url = format!("https://adventofcode.com/{year}/day/{d}/input");
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?;
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(session)?);
    return match client.get(url).headers(headers).send() {
        Ok(response) => Ok(response.text()?),
        Err(e) => Err(Box::new(e))
    }
}

pub fn validate_unit(unit: &str) -> Result<AdventUnit, String> {
    let day_reg = Regex::new(r"^(0[1-9]|1[0-9]|2[0-5])$").unwrap();
    let year_reg = Regex::new(r"^(201[5-9]|202[0-9])$").unwrap();
    if day_reg.is_match(unit) {
        Ok(AdventUnit::Day(String::from(unit)))
    } else if year_reg.is_match(unit){
        Ok(AdventUnit::Year(String::from(unit)))
    } else {
        Err(String::from(
            "Either provide a zero-padded string in the range 01-25 for a new day, or a year from 2015 on.",
        ))
    }
}

pub fn validate_day(unit: &str) -> Result<String, String> {
    let day_reg = Regex::new(r"^(0[1-9]|1[0-9]|2[0-5])$").unwrap();
    if day_reg.is_match(unit) {
        Ok(String::from(unit))
    } else {
        Err(String::from(
            "Provide a zero-padded string in the range 01-25.",
        ))
    }
}

pub fn validate_year(unit: &str) -> Result<String, String> {
    let year_reg = Regex::new(r"^(201[5-9]|202[0-9])$").unwrap();
    if year_reg.is_match(unit) {
        Ok(String::from(unit))
    } else {
        Err(String::from(
            "Provide a value between 2015 and 2029.",
        ))
    }
}

