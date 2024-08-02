use std::collections::HashMap;
use std::error::Error;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};
use select::document::Document;
use select::predicate::Name;
use crate::Config;

pub fn submit(year: &str, day: &str, part: u8, cfg: &Config) {
    if let Some(solution) = find_solution(year, day, &cfg.solutions, part) {
        println!("Submitting '{solution}' as the solution for Part {part} of Day {day} of AoC {year}");
        match post_solution(year, day, &cfg.get_session(), solution, &part.to_string()) {
            Ok(response) => {
                if let Some(article) = Document::from(response.as_str()).find(Name("article")).next() {
                    if let Some(p) = article.find(Name("p")).next() {
                        println!("{}", p.text());
                    }
                }
            }
            Err(_) => {eprintln!("Error during submission.")}
        }
    }
}

fn post_solution(year: &str, day: &str, session: &str, solution: String, part: &str) -> Result<String, Box<dyn Error>> {
    let d = day.parse::<i32>().unwrap();
    let url = format!("https://adventofcode.com/{year}/day/{d}/answer");
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?;
    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, HeaderValue::from_str(session)?);
    let mut form_data: HashMap<&str, &str> = HashMap::new();
    form_data.insert("level", part);
    form_data.insert("answer", &solution);

    return match client.post(url).form(&form_data).headers(headers).send() {
        Ok(response) => Ok(response.text()?),
        Err(e) => Err(Box::new(e))
    }
}

fn find_solution(year: &str, day: &str, solutions: &HashMap<String, Vec<String>>, part: u8) -> Option<String> {
    if let Some(solved) = solutions.get(&format!("{year}-{day}")) {
        if let Some(submission) = solved.get((part - 1) as usize) {
            Some(submission.clone())
        } else {
            eprintln!("There is a solution for day {day} of AoC {year}, but not for Part {part}");
            None
        }
    } else {
        eprintln!("There is no solution for day {day} of AoC {year} to be submitted.");
        None
    }
}
