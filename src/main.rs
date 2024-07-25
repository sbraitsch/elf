mod submit;
mod bootstrap;
mod utils;

use std::collections::HashMap;
use std::{env, fs};
use clap::{Parser, Subcommand, ValueEnum};
use jiff::Zoned;
use serde::{Deserialize, Serialize};
use crate::bootstrap::go_project::GoProject;
use crate::bootstrap::rust_project::RustProject;
use crate::bootstrap::traits::Bootstrap;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    year: String,
    day: String,
    lang: String,
    solutions: HashMap<String, Vec<String>>
}

#[derive(ValueEnum, Debug, Clone)]
enum Language {
    Rust,
    Go,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    Bootstrap {
        #[arg(short, long, value_name = "YEAR", default_value = current_year(), help = "Specify the year you want to get started with")]
        year: String,
        #[arg(short, long, required = true, value_name = "LANGUAGE", help = "Specify language to use. Supports [rust, go]")]
        lang: Language,
        name: String
    },
    Submit {
        #[arg(short, long, required = false, value_name = "YEAR", help = "Specify the year")]
        year: Option<String>,
        #[arg(short, long, required = false, value_name = "DAY", help = "Specify the day")]
        day: Option<i8>,
        #[arg(short, long, value_name = "PART", default_value_t = 1, help = "Specify the solution part")]
        part: u8,
    },
    New {
        #[arg(short, long, required = false, value_name = "YEAR", help = "Specify the year")]
        year: Option<String>,
        #[arg(short, long, required = false, value_name = "DAY", help = "Specify the day")]
        day: Option<i8>,
        #[arg(short, long, required = false, value_name = "FILE", help = "Specify the template file to use")]
        template: Option<String>,
    },
}

fn main() {
    // ---------DEV----------
    env::set_current_dir("../").expect("Error moving to project dir");
    // ----------------------
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Bootstrap { year , lang, name}) => {
            let mut cfg = Config{
                year: year.clone(),
                day: String::from("01"),
                lang: "".to_string(),
                solutions: Default::default(),
            };
            let res = match lang {
                Language::Rust => {
                    cfg.lang.push_str("rust");
                    RustProject{}.project(year, name, &mut cfg)
                },
                Language::Go => {
                    cfg.lang.push_str("go");
                    GoProject{}.project(year, name, &mut cfg)
                }
            };

            match res {
                Ok(_) => {},
                Err(e) => eprintln!("Error during Bootstrap Process: {e:?}")
            }
        }
        Some(Commands::Submit { year, day, part }) => {
            if let Some(mut cfg) = read_config() {
                find_solution(part, &cfg.year, &cfg.day, &mut cfg.solutions);
                match (year, day) {
                    (Some(_), None) | (None, Some(_)) => eprintln!("Specify either both year and day, or none"),
                    (Some(y), Some(d)) => {
                        let solution = find_solution(part, y, &fmt_day(*d), &mut cfg.solutions).expect(&format!("No computed solution found for {y}-{d}"));
                        submit::submit(y, *d, *part, &solution)
                    },
                    (None, None) => {
                        let solution = find_solution(part, &cfg.year, &cfg.day, &mut cfg.solutions).expect(&format!("No computed solution found for {}-{}", cfg.year, cfg.day));
                        submit::submit(&cfg.year, cfg.day.parse::<i8>().unwrap(), *part, &solution)
                    },
                }
            }
        }
        Some(Commands::New { year, day, template: _template}) => {
            // -------DEV-------
            env::set_current_dir("aoc_test").expect("Unexpected directory");
            // -----------------
            if let Some(mut cfg) = read_config() {
                let project: Box<dyn Bootstrap>= match cfg.lang.as_str() {
                    "rust" => Box::new(RustProject{}),
                    "go"   => Box::new(GoProject {}),
                    _ => panic!()
                };
                let _res = match (year, day) {
                    (Some(y), None) => project.module(y, &mut cfg),
                    (None, Some(d)) => project.day(&cfg.year.clone(), &fmt_day(*d), &mut cfg),
                    (Some(y), Some(d)) => project.day(y, &fmt_day(*d), &mut cfg),
                    (None, None) => {
                        let day_num = &cfg.day.parse::<i8>().unwrap() + 1;
                        project.day(&cfg.year.clone(), &fmt_day(day_num), &mut cfg)
                    },
                };
            }
        }
        None => unreachable!("A subcommand is required"), // clap ensures this
    }
}

fn find_solution(part: &u8, year: &String, day: &String, solutions: &mut HashMap<String, Vec<String>>) -> Option<String> {
    if let Some(solved) = solutions.get(&format!("{year}-{day}")) {
        if let Some(submission) = solved.get((*part - 1) as usize) {
            Some(submission.clone())
        } else {
            None
        }
    } else {
        None
    }
}

fn read_config() -> Option<Config> {
    if let Ok(content) = fs::read_to_string("elf.toml") {
        let config = toml::from_str(&content).expect("Error parsing elf.toml");
        Some(config)
    } else {
        eprintln!("Couldn't find elf.toml. Are you in the correct project root?");
        None
    }
}

fn fmt_day( day: i8) -> String {
    return if day < 10 {
        format!("0{day}")
    } else {
        day.to_string()
    };
}

fn current_year() -> String {
    Zoned::now().year().to_string()
}
