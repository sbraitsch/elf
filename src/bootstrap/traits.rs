use std::error::Error;
use crate::Config;

pub trait Bootstrap {
    fn project(&self, year: &str, name: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>>;

    fn module(&self, year: &str, cfg: &mut Config) -> Result<(), Box<dyn Error>>;

    fn day(&self, year: &str, day: &str, cfg :&mut Config) -> Result<(), Box<dyn Error>>;
}