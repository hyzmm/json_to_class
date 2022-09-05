use serde_json::{Map, Value};

pub mod dart_generator;

pub trait ClassGenerator {
    fn parse_value(&mut self, value: &Value) -> String;
    fn parse_array(&mut self, value: &[Value]) -> String;
    fn parse_object(&mut self, value: &Map<String, Value>) -> &str;
    fn get_full_result(self) -> String;
}
