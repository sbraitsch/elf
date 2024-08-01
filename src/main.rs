mod config;
mod scaffold;
mod submit;
mod utils;

use std::{env, io};
use std::path::Path;
use crate::config::{Config, Language};
use crate::scaffold::traits::Scaffold;
use crate::utils::{read_config, update_elf};
use clap::{Parser, Subcommand, value_parser};
use regex::Regex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    New {
        #[arg(
            short,
            long,
            value_name = "LANGUAGE",
            default_value = "rust",
            value_parser = value_parser!(Language),
            help = "Specify language to use. Supports [rust, kotlin, c++, go]"
        )]
        lang: Language,
        name: String,
    },
    Submit {
        #[arg(
            short,
            long,
            required = false,
            value_name = "YEAR",
            help = "Specify the year"
        )]
        year: Option<String>,
        #[arg(short, long, required = false, value_parser = validate_day, value_name = "DAY", help = "Specify the day")]
        day: Option<String>,
        #[arg(
            short,
            long,
            value_name = "PART",
            default_value_t = 1,
            help = "Specify the solution part"
        )]
        part: u8,
    },
    Add {
        #[arg(
            short,
            long,
            required = false,
            value_name = "YEAR",
            help = "Specify the year"
        )]
        year: Option<String>,
        #[arg(short, long, required = false, value_parser = validate_day, value_name = "DAY", help = "Specify the day")]
        day: Option<String>,
    },
    Set {
        #[arg(
            short,
            long,
            required = false,
            value_name = "YEAR",
            help = "Specify the year"
        )]
        year: Option<String>,
        #[arg(short, long, required = false, value_parser = validate_day, value_name = "DAY", help = "Specify the day")]
        day: Option<String>,
        #[arg(
            short,
            long,
            required = false,
            value_name = "SESSION_TOKEN",
            help = "Your session token for AoC, including session="
        )]
        session: Option<String>,
        #[arg(
            short,
            long,
            required = false,
            value_name = "TEMPLATE_PATH",
            help = "Path to your custom template, relative to project root."
        )]
        template: Option<String>,
    },
    Next,
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
        Some(Commands::New { lang, name }) => {
            if Path::new(name).exists() {
                eprintln!("There is already a directory called {name}. Try a different name.");
                return;
            }
            println!("A diligent elf has started scaffolding your new project: \'{name}\'🎁");
            println!("Please enter your AoC session token:");
            let mut session_token = String::new();
            io::stdin().read_line(&mut session_token).unwrap();

            match lang.to_project().project(name, session_token.trim().to_string()) {
                Ok(_) => println!("You're all set.\nTo get started, create a new module with: 'elf add -y=<YEAR>'"),
                Err(e) => eprintln!("Error during scaffolding process: {e:?}"),
            }
        }
        Some(Commands::Submit { year, day, part }) => {
            if let Some(cfg) = read_config() {
                match (year, day) {
                    (Some(_), None) | (None, Some(_)) => {
                        eprintln!("Specify either both year and day, or none")
                    }
                    (Some(y), Some(d)) => submit::submit(y, d, *part, &cfg),
                    (None, None) => submit::submit(&cfg.year, &cfg.day, *part, &cfg),
                }
            }
        }
        Some(Commands::Add { year, day }) => {
            if let Some(mut cfg) = read_config() {
                let project: Box<dyn Scaffold> = cfg.lang.to_project();
                match (year, day) {
                    (Some(y), None) => project
                        .module(y, &mut cfg)
                        .expect(&format!("Error creating new module for AoC {y}")),
                    (None, Some(d)) => project.day(&cfg.year.clone(), d, &mut cfg).expect(
                        &format!("Error creating stubs for AoC {}, day {d}", &cfg.year),
                    ),
                    (Some(_), Some(_)) | (None, None) => {
                        eprintln!("Add only supports year XOR day.")
                    }
                };
            }
        }
        Some(Commands::Next) => {
            if let Some(mut cfg) = read_config() {
                let project: Box<dyn Scaffold> = cfg.lang.to_project();
                let day_num = &cfg.day.parse::<i8>().unwrap() + 1;
                if let Err(e) = project.day(&cfg.year.clone(), &fmt_day(day_num), &mut cfg) {
                    eprintln!("Error during stub generation: {e:?}");
                }
            }
        }
        Some(Commands::Set {
            year,
            day,
            session,
            template,
        }) => {
            if let Some(mut cfg) = read_config() {
                update_elf(
                    year.clone(),
                    day.clone(),
                    session.clone(),
                    template.clone(),
                    &mut cfg,
                )
                .expect("Error updating elf.toml");
            }
        }
        None => unreachable!("A subcommand is required"), // clap ensures this
    }
}

fn fmt_day(day: i8) -> String {
    return if day < 10 {
        format!("0{day}")
    } else {
        day.to_string()
    };
}

fn validate_day(day: &str) -> Result<String, String> {
    let re = Regex::new(r"^(0[1-9]|1[0-9]|2[0-5])$").unwrap();
    if re.is_match(day) {
        Ok(String::from(day))
    } else {
        Err(String::from(
            "The day must be a zero-padded string in the range 01-25",
        ))
    }
}
