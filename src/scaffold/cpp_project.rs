#![allow(unused)]
use std::error::Error;
use crate::Config;
use crate::scaffold::projects::CppProject;
use super::traits::Scaffold;
impl Scaffold for CppProject {
    fn project(&self, name: &str, token: String) -> Result<(), Box<dyn Error>> {
        println!("Cpp support is under development");
        Ok(())
    }

    fn module(&self, year: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        println!("Cpp support is under development");
        Ok(())
    }

    fn day(&self, year: &str, day: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>> {
        println!("Cpp support is under development");
        Ok(())
    }
}
