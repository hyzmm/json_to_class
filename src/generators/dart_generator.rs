use convert_case::{Case, Casing};
use serde_json::{Map, Value};

use crate::generators::ClassGenerator;

pub struct DartClassGenerator {
    class_name: String,
    fields: Vec<(String, String)>,
}

impl DartClassGenerator {
    pub fn new(class_name: &str) -> DartClassGenerator {
        DartClassGenerator {
            class_name: class_name.to_string(),
            fields: Vec::new(),
        }
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

    fn parse_array(&mut self, value: &Vec<Value>) -> String {
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

    fn parse_object(&mut self, obj: &Map<String, Value>) -> &'static str {
        for (k, v) in obj.iter() {
            let type_name = if v.is_object() {
                let class_name = k.to_case(Case::Pascal);
                // self.nested_classes.insert(class_name, v);
                class_name
            } else {
                self.parse_value(v)
            };
            self.fields
                .push((k.to_string().to_case(Case::Camel), type_name.to_string()));
        }
        "dynamic"
    }

    fn get_result(self) -> String {
        let body = self
            .fields
            .iter()
            // always `final` for now
            .map(|(k, v)| format!("    final {} {};", v, k))
            .collect::<Vec<String>>()
            .join("\n");

        let constructor_args = self
            .fields
            .iter()
            .map(|(k, _)| format!("        required this.{},", k))
            .collect::<Vec<String>>()
            .join("\n");

        format!(
            r#"import 'package:json_annotation/json_annotation.dart';

part '{file_name}.g.dart';

@JsonSerializable()
class {class_name} {{
{body}
    {class_name}({{
{constructor_args}
    }});

    factory {class_name}.fromJson(Map<String, dynamic> json) => _${class_name}FromJson(json);

    Map<String, dynamic> toJson() => _${class_name}ToJson(this);
}}"#,
            file_name = self.class_name.to_case(Case::Snake),
            class_name = self.class_name,
            body = body,
            constructor_args = constructor_args,
        )
    }
}
