use std::collections::HashMap;

use convert_case::{Case, Casing};
use serde_json::{Map, Value};

use crate::generators::ClassGenerator;

#[derive(PartialEq, Eq, Clone)]
pub enum FieldType {
    BaseType(String),
    Class(DartClassGenerator),
}

impl FieldType {
    fn get_type(&self) -> String {
        match self {
            FieldType::BaseType(value) => value.clone(),
            FieldType::Class(class) => class.class_name.clone(),
        }
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct DartClassGenerator {
    class_name: String,
    fields: Vec<(String, FieldType)>,
    classes: Vec<DartClassGenerator>,
}

impl DartClassGenerator {
    pub fn new(class_name: &str) -> DartClassGenerator {
        DartClassGenerator {
            class_name: class_name.to_string(),
            fields: Vec::new(),
            classes: Vec::new(),
        }
    }
}

impl DartClassGenerator {
    fn get_classes_recursively(&self) -> Vec<DartClassGenerator> {
        let mut result = Vec::new();
        result.push(self.clone());
        for class in &self.classes {
            result.push(class.clone());
            result.append(&mut class.get_classes_recursively());
        }
        result
    }

    fn get_result(&self, override_class_name: Option<String>) -> String {
        let class_name = override_class_name.unwrap_or_else(|| self.class_name.clone());
        let body = self
            .fields
            .iter()
            // always `final` for now
            .map(|(k, v)| format!("    final {} {};", v.get_type(), k))
            .collect::<Vec<String>>()
            .join("\n");

        let constructor_args = self
            .fields
            .iter()
            .map(|(k, _)| format!("        required this.{},", k))
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            r#"@JsonSerializable()
class {class_name} {{
{body}
    {class_name}({{
{constructor_args}
    }});

    factory {class_name}.fromJson(Map<String, dynamic> json) => _${class_name}FromJson(json);

    Map<String, dynamic> toJson() => _${class_name}ToJson(this);
}}"#,
            class_name = class_name,
            body = body,
            constructor_args = constructor_args,
        )
    }
}

impl ClassGenerator for DartClassGenerator {
    fn parse_value(&mut self, value: &Value) -> String {
        let decide_num_type = |num: &Value| {
            assert!(value.is_number());
            if num.is_i64() {
                "int"
            } else if num.is_f64() {
                "double"
            } else {
                panic!("Unknown number type");
            }
        };

        match value {
            Value::Null => "dynamic".to_string(),
            Value::Bool(_) => "bool".to_string(),
            Value::Number(_) => decide_num_type(value).to_string(),
            Value::String(_) => "String".to_string(),
            Value::Array(arr) => self.parse_array(arr),
            Value::Object(obj) => self.parse_object(obj).to_string(),
        }
    }

    fn parse_array(&mut self, value: &[Value]) -> String {
        if value.is_empty() {
            return "dynamic".to_string();
        }

        let mut result: Option<String> = None;
        for i in value {
            let t = self.parse_value(i);
            if result.is_none() {
                result = Some(t);
            } else if result != Some(t) {
                result = Some("dynamic".to_string());
            }
        }
        format!("List<{}>", result.unwrap())
    }

    fn parse_object(&mut self, obj: &Map<String, Value>) -> &str {
        for (k, v) in obj.iter() {
            // remove illegal characters and leading numbers
            let k = k
                .trim_start_matches(|c: char| c.is_numeric())
                .replace(|c: char| !c.is_alphanumeric(), "")
                .to_string();
            if v.is_object() {
                let class_name = k.to_case(Case::Pascal);
                let mut generator = DartClassGenerator::new(class_name.clone().as_ref());
                generator.parse_value(v);
                self.classes.push(generator.clone());
                self.fields.push((k.to_case(Case::Camel), FieldType::Class(generator.clone())));
            } else {
                let t = self.parse_value(v);
                self.fields.push((k.to_case(Case::Camel), FieldType::BaseType(t)));
            };
        }
        "dynamic"
    }

    fn get_full_result(self) -> String {
        let mut classes_string: Vec<String> = Vec::new();
        let mut generated_classes: HashMap<String, (DartClassGenerator, usize)> = HashMap::new();

        let mut classes = self.get_classes_recursively();
        classes.push(self.clone());

        for class in classes {
            let name = class.class_name.clone();

            if generated_classes.contains_key(&name)
                && generated_classes[&name].0 == class {
                continue;
            }

            // insert if not exists, or increment if exists
            generated_classes.entry(name.clone())
                .and_modify(|e| e.1 += 1)
                .or_insert((class.clone(), 0));

            let name = if generated_classes[&name.clone()].1 > 0 {
                Some(format!("{}{}", name, generated_classes[&name.clone()].1))
            } else {
                None
            };
            classes_string.push(class.get_result(name));
        }

        format!(
            r#"import 'package:json_annotation/json_annotation.dart';

part '{file_name}.g.dart';

{classes_string}
"#,
            file_name = self.class_name.to_case(Case::Snake),
            classes_string = classes_string.join("\n\n"),
        )
    }
}
