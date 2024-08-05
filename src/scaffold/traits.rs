use std::error::Error;
use crate::Config;

pub trait Scaffold {
    fn project(&self, name: &str, token: String) -> Result<(), Box<dyn Error>>;

    fn module(&self, year: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>>;

    fn day(&self, year: &str, day: &str, cfg :&mut Config) -> Result<(), Box<dyn Error>>;

    fn run(&self, release: bool, cfg: &mut Config) -> Result<(), Box<dyn Error>>;
}