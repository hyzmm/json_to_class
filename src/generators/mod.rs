use convert_case::{Case, Casing};
use serde_json::{Map, Value};

pub mod dart_generator;

pub trait ClassGenerator {
    fn parse_value(&mut self, value: &Value) -> String;
    fn parse_array(&mut self, value: &[Value]) -> String;
    fn parse_object(&mut self, value: &Map<String, Value>) -> &str;
    fn get_full_result(self) -> String;
}

pub fn to_legal_case(name: &str, case: Case) -> String {
    name
        .trim_start_matches(|c: char| !c.is_alphabetic())
        .to_case(case)
        .replace(|c: char| !c.is_alphanumeric(), "")
}

#[derive(clap::ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum NamingRule {
    None,
    Snake,
    Pascal,
    Kebab,
}

impl Into<Case> for NamingRule {
    fn into(self) -> Case {
        match self {
            NamingRule::None => Case::Snake,
            NamingRule::Snake => Case::Snake,
            NamingRule::Pascal => Case::Pascal,
            NamingRule::Kebab => Case::Kebab,
        }
    }
}