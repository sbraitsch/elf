mod submit;
mod scaffold;
mod utils;

use std::collections::HashMap;
use std::env;
use clap::{Parser, Subcommand, ValueEnum};
use jiff::Zoned;
use regex::Regex;
use serde::{Deserialize, Serialize};
use crate::scaffold::go_project::GoProject;
use crate::scaffold::rust_project::RustProject;
use crate::scaffold::traits::Scaffold;
use crate::utils::{read_config, update_elf};

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
    New {
        #[arg(short, long, value_name = "YEAR", default_value = current_year(), help = "Specify the year you want to get started with")]
        year: String,
        #[arg(short, long, required = true, value_name = "LANGUAGE", help = "Specify language to use. Supports [rust, go]")]
        lang: Language,
        name: String
    },
    Submit {
        #[arg(short, long, required = false, value_name = "YEAR", help = "Specify the year")]
        year: Option<String>,
        #[arg(short, long, required = false, value_parser = validate_day, value_name = "DAY", help = "Specify the day")]
        day: Option<String>,
        #[arg(short, long, value_name = "PART", default_value_t = 1, help = "Specify the solution part")]
        part: u8,
    },
    Add {
        #[arg(short, long, required = false, value_name = "YEAR", help = "Specify the year")]
        year: Option<String>,
        #[arg(short, long, required = false, value_parser = validate_day, value_name = "DAY", help = "Specify the day")]
        day: Option<String>,
        #[arg(short, long, required = false, value_name = "FILE", help = "Specify the template file to use")]
        template: Option<String>,
    },
    Set {
        #[arg(short, long, required = false, value_name = "YEAR", help = "Specify the year")]
        year: Option<String>,
        #[arg(short, long, required = false, value_parser = validate_day, value_name = "DAY", help = "Specify the day")]
        day: Option<String>,
    },
    Next
}

fn main() {
    let cli = Cli::parse();

    // ---------DEV----------
    if let Some(Commands::New { .. }) = &cli.command {
        env::set_current_dir("../").expect("Error moving to project dir");
    } else {
        env::set_current_dir("../aoc").expect("Error moving to project dir");
    }
    // ----------------------

    match &cli.command {
        Some(Commands::New { year , lang, name}) => {
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
                Err(e) => eprintln!("Error during scaffolding process: {e:?}")
            }
        }
        Some(Commands::Submit { year, day, part }) => {
            if let Some(cfg) = read_config() {
                match (year, day) {
                    (Some(_), None) | (None, Some(_)) => eprintln!("Specify either both year and day, or none"),
                    (Some(y), Some(d)) => submit::submit(y, d, *part, &cfg),
                    (None, None) => submit::submit(&cfg.year, &cfg.day, *part, &cfg)
                }
            }
        }
        Some(Commands::Add { year, day, template: _template}) => {
            if let Some(mut cfg) = read_config() {
                let project: Box<dyn Scaffold> = get_builder(&cfg);
                match (year, day) {
                    (Some(y), None) => project.module(y, &mut cfg).expect(&format!("Error creating new module for AoC {y}")),
                    (None, Some(d)) => project.day(&cfg.year.clone(), d, &mut cfg).expect(&format!("Error creating stubs for AoC {}, day {d}", &cfg.year)),
                    (Some(_), Some(_)) |  (None, None) => eprintln!("Add only supports year XOR day."),
                };
            }
        }
        Some(Commands::Next) => {
            if let Some(mut cfg) = read_config() {
                let project: Box<dyn Scaffold> = get_builder(&cfg);
                let day_num = &cfg.day.parse::<i8>().unwrap() + 1;
                project.day(&cfg.year.clone(), &fmt_day(day_num), &mut cfg).expect("Error creating stubs for the next puzzle");
            }
        },
        Some(Commands::Set {year, day}) => {
            if let Some(mut cfg) = read_config() {
                let _res = match (year, day) {
                    (Some(year), None) => update_elf(year, &cfg.day.clone(), &mut cfg),
                    (None, Some(day)) => update_elf(&cfg.year.clone(), day, &mut cfg),
                    (Some(year), Some(day)) => update_elf(year, day, &mut cfg),
                    (None, None) => Ok(()),
                };
            }
        }
        None => unreachable!("A subcommand is required"), // clap ensures this
    }
}

fn get_builder(cfg: &Config) -> Box<dyn Scaffold> {
    match cfg.lang.as_str() {
        "rust" => Box::new(RustProject {}),
        "go" => Box::new(GoProject {}),
        _ => panic!()
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

fn validate_day(day: &str) -> Result<String, String> {
    let re = Regex::new(r"^(0[1-9]|1[0-9]|2[0-5])$").unwrap();
    if re.is_match(day) {
        Ok(String::from(day))
    } else {
        Err(String::from("The day must be a zero-padded string in the range 01-25"))
    }
}
