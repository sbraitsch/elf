#![allow(unused)]
use std::error::Error;
use crate::Config;
use crate::scaffold::projects::KotlinProject;
use super::traits::Scaffold;

impl Scaffold for KotlinProject {
    fn project(&self, name: &str, token: String) -> Result<(), Box<dyn Error>> {
        println!("Kotlin support is under development");
        Ok(())
    }

    fn module(&self, year: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        println!("Kotlin support is under development");
        Ok(())
    }

    fn day(&self, year: &str, day: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        println!("Kotlin support is under development");
        Ok(())
    }

    fn run(&self, release: bool, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
