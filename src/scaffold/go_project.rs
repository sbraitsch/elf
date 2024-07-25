#![allow(unused)]
use std::error::Error;
use crate::Config;
use super::traits::Scaffold;

pub struct GoProject {}
impl Scaffold for GoProject {
    fn project(&self, year: &str, name: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
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
}
