use crate::Config;
use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

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
    println!("created: {path:?}");
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
    println!("modified: {path:?}");
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
    println!("modified: elf.toml");
    Ok(())
}
