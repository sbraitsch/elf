use std::{collections::HashMap};
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::scaffold::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub year: String,
    pub day: String,
    pub lang: Language,
    pub session: String,
    pub template: Option<String>,
    pub solutions: HashMap<String, Vec<String>>,
}

impl Config {
    pub fn new(lang: Language, session: String) -> Self {
        Config{
            year: "Not Set".to_string(),
            day: "Not Set".to_string(),
            lang,
            session,
            template: None,
            solutions: Default::default(),
        }
    }

    pub fn get_session(&self) -> String {
        let mut token = String::new();
        if !self.session.starts_with("session=") {
            token.push_str("session=");
        }
        token.push_str(&self.session);
        token
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Language {
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "kotlin")]
    Kotlin,
    #[serde(rename = "c++")]
    Cpp,
    #[serde(rename = "go")]
    Go,
}

impl Language {
    pub fn to_project(&self) -> Box<dyn Scaffold> {
        match self {
            Self::Rust => Box::new(RustProject {}),
            Self::Go => Box::new(GoProject {}),
            Self::Kotlin => Box::new(KotlinProject{}),
            Self::Cpp => Box::new(CppProject{})
        }
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "c++" | "cpp" | "cc" => Ok(Language::Cpp),
            "rust" | "rs" => Ok(Language::Rust),
            "kotlin" | "kt" => Ok(Language::Kotlin),
            "go" | "golang" => Ok(Language::Go),
            _ => Err(format!("'{s}' is not a supported language."))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AdventUnit {
    Day(String),
    Year(String)
}
