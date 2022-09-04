extern crate core;

use serde_json::{Result, Value};

use crate::generators::ClassGenerator;

pub mod generators;

pub fn json_to_class(json_string: &str, mut generator: impl ClassGenerator) -> Result<String> {
    let v: Value = serde_json::from_str(json_string)?;
    generator.parse_value(&v);

    Ok(generator.get_result())
}
