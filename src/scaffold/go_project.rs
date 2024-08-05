#![allow(unused)]
use std::error::Error;
use crate::Config;
use crate::scaffold::projects::GoProject;
use super::traits::Scaffold;

impl Scaffold for GoProject {
    fn project(&self, name: &str, token: String) -> Result<(), Box<dyn Error>> {
        println!("Golang support is not yet implemented");
        Ok(())
    }

    fn module(&self, year: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        println!("Golang support is not yet implemented");
        Ok(())
    }

    fn day(&self, year: &str, day: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        println!("Golang support is not yet implemented");
        Ok(())
    }

    fn run(&self, release: bool, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
