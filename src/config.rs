use crate::{scaffold::go_project::GoProject, scaffold::rust_project::RustProject, Scaffold};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub year: String,
    pub day: String,
    pub lang: Language,
    pub session: String,
    pub template: Option<String>,
    pub solutions: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, ValueEnum, Debug, Clone)]
pub enum Language {
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "go")]
    Go,
}

impl Language {
    pub fn to_project(&self) -> Box<dyn Scaffold> {
        match self {
            Self::Rust => Box::new(RustProject {}),
            Self::Go => Box::new(GoProject {}),
        }
    }
}
