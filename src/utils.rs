use std::error::Error;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use crate::Config;

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

pub fn update_elf(year: &str, day: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
    cfg.day = String::from(day);
    cfg.year = String::from(year);
    let elf = toml::ser::to_string(&cfg)?;
    fs::write("elf.toml", elf)?;
    println!("modified: elf.toml");
    Ok(())
}
