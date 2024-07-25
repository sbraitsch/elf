mod template;
mod bootstrap;
mod submit;

use std::collections::HashMap;
use std::{env, fs};
use clap::{Parser, Subcommand};
use jiff::Zoned;
use serde::Deserialize;
use crate::bootstrap::bootstrap;

#[derive(Deserialize, Debug)]
struct Config {
    year: String,
    day: String,
    solutions: HashMap<String, Vec<String>>
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
    // -------------------
    env::set_current_dir("../aoc/").expect("Error moving to project dir");
    // -------------------
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Bootstrap { year }) => {
            bootstrap(year);
        }
        Some(Commands::Submit { year, day, part }) => {
            if let Some(Config { year: cfg_year, day: cfg_day, mut solutions }) = read_config() {
                find_solution(part, &cfg_year, &cfg_day, &mut solutions);
                match (year, day) {
                    (Some(_), None) | (None, Some(_)) => eprintln!("Specify either both year and day, or none"),
                    (Some(y), Some(d)) => {
                        let solution = find_solution(part, y, &fmt_day(*d), &mut solutions).expect(&format!("No computed solution found for {y}-{d}"));
                        submit::submit(y, *d, *part, &solution)
                    },
                    (None, None) => {
                        let solution = find_solution(part, &cfg_year, &cfg_day, &mut solutions).expect(&format!("No computed solution found for {cfg_year}-{cfg_day}"));
                        submit::submit(&cfg_year, cfg_day.parse::<i8>().unwrap(), *part, &solution)
                    },
                }
            }
        }
        Some(Commands::New { year, day, template: _template}) => {
            if let Some(Config { year: cfg_year, day: cfg_day, solutions: _}) = read_config() {
                match (year, day) {
                    (Some(y), None) => template::new_year(y),
                    (None, Some(d)) => template::new_day(&cfg_year, &fmt_day(*d)),
                    (Some(y), Some(d)) => template::new_day(y, &fmt_day(*d)),
                    (None, None) => {
                        let day_num = &cfg_day.parse::<i8>().unwrap() + 1;
                        template::new_day(&cfg_year, &fmt_day(day_num))
                    },
                }
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
        println!("{config:?}");
        Some(config)
    } else {
        eprintln!("elf.toml not found");
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
