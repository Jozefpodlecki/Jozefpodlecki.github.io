use std::{error::Error, fmt::{self, Debug, Display, Formatter}};

use serde::Deserialize;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Summary {
    pub full_name: String,
    pub role: String,
    pub social: Social,
    pub projects: Vec<Project>,
    pub experience: Vec<Experience>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Social {
    pub linkedin: String,
    pub github: String,
    pub duolingo: String,
    pub codepen: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub link: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Experience {
    pub company: String,
    pub location: String,
    pub employment_type: String,
    pub work_mode: String,
    pub role: String,
    pub period: String,
    pub responsibilities: Vec<String>,
}
