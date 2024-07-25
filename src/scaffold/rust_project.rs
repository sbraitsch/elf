use std::{env, fs};
use std::error::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue};
use crate::Config;
use crate::utils::{update_elf, write_new_file, write_to_file};
use super::traits::Scaffold;

const UTILS: &str = include_str!("../templates/utils.rs");
const TEMPLATE: &str = include_str!("../templates/template.rs");
pub struct RustProject {}

impl Scaffold for RustProject {
    fn project(&self, year: &str, name: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        if Path::new(name).exists() {
            let err = std::io::Error::new(ErrorKind::AlreadyExists, "Project directory already exists");
            return Err(Box::new(err));
        }
        let cmd = Command::new("cargo")
            .arg("new")
            .arg(name)
            .output()?;
        if cmd.status.success() {
            println!("A diligent elf is scaffolding your new project: \'{name}\'🎁");
            env::set_current_dir(name)?;
            write_new_file(Path::new("session.txt"), "session=<YOURSESSION>")?;
            let git_ignore = "**/inputs/\nsession.txt";
            write_to_file(Path::new(".gitignore"), git_ignore)?;
            write_new_file(Path::new("src/utils.rs"), UTILS)?;
            self.module(year, cfg)?;
            println!("You're all set. Have fun! 🎅🏻");
            Ok(())
        } else {
            let err = std::io::Error::new(ErrorKind::Other, String::from_utf8_lossy(&cmd.stderr));
            return Err(Box::new(err));
        }
    }

    fn module(&self, year: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        fs::create_dir_all(format!("src/aoc_{year}/solutions"))?;
        fs::create_dir_all(format!("src/aoc_{year}/inputs"))?;
        let aoc_mod = "pub mod solutions;\npub use solutions::*;";
        write_new_file(Path::new(&format!("src/aoc_{year}/mod.rs")), aoc_mod)?;
        self.day(year, "01", cfg)?;
        Ok(())
    }

    fn day(&self, year: &str, day: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        let base_path = format!("src/aoc_{year}");
        if !Path::new(&base_path).exists() {
            let err = std::io::Error::new(ErrorKind::NotFound, format!("No module found for AoC {year}. Create one first with elf add -y=xxxx"));
            return Err(Box::new(err));
        }
        update_elf(year, day, cfg)?;

        if let Err(e) = write_solution_template(&base_path, year, day) {
            eprintln!("Error writing solution template: {}", e);
            std::process::exit(1);
        }
        if let Err(e) = write_solution_mod(&base_path, day) {
            eprintln!("Error updating mod.rs: {}", e);
            std::process::exit(1);
        }
        let session;
        if let Ok(s) = fs::read_to_string("session.txt") {
            session = s;
        } else {
            let err = std::io::Error::new(ErrorKind::AlreadyExists, "No session.txt found. Input will not be retrievable.");
            return Err(Box::new(err));
        }
        if let Err(e) = write_input(&base_path, year, day, &session) {
            eprintln!("Error writing input file: {}", e);
            std::process::exit(1);
        }
        Ok(())
    }
}


fn write_solution_template(base_path: &str, year: &str, day: &str) -> Result<(), Box<dyn Error>> {
    let content = TEMPLATE.replace("{{year}}", year).replace("{{day}}", day);
    let filename = format!("day_{day}.rs");
    let file_path = Path::new(&base_path).join("solutions").join(&filename);
    write_new_file(&file_path, &content)?;
    Ok(())
}

fn write_solution_mod(base_path: &str, day: &str) -> Result<(), Box<dyn Error>> {
    let mod_path = Path::new(&base_path).join("solutions").join("mod.rs");
    let mut content = if mod_path.exists() {
        fs::read_to_string(&mod_path)?
    } else {
        String::new()
    };
    let add_to_mod = format!("pub mod.rs day_{};\n", day);
    if !content.contains(&add_to_mod) {
        content.push_str(&add_to_mod);
        fs::write(&mod_path, content)?;
        println!("modified: {mod_path:?}")
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
    write_new_file(&file_path, &response.text()?)?;
    Ok(())
}

