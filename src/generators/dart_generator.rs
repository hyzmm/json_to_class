use convert_case::{Case, Casing};
use serde_json::{Map, Value};

use crate::generators::ClassGenerator;

pub struct DartClassGenerator {
    class_name: String,
    fields: Vec<(String, String)>,
    nested_classes: Vec<String>,
}

impl DartClassGenerator {
    pub fn new(class_name: &str) -> DartClassGenerator {
        DartClassGenerator {
            class_name: class_name.to_string(),
            fields: Vec::new(),
            nested_classes: Vec::new(),
        }
    }
}

impl DartClassGenerator {
    fn get_result(self) -> Vec<String> {
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

        let class = format!(
            r#"@JsonSerializable()
class {class_name} {{
{body}
    {class_name}({{
{constructor_args}
    }});

    factory {class_name}.fromJson(Map<String, dynamic> json) => _${class_name}FromJson(json);

    Map<String, dynamic> toJson() => _${class_name}ToJson(this);
}}"#,
            class_name = self.class_name,
            body = body,
            constructor_args = constructor_args,
        );
        let mut result = Vec::new();
        result.push(class);
        result.extend(self.nested_classes);
        result
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
            let type_name = if v.is_object() {
                let class_name = k.to_case(Case::Pascal);
                let mut generator = DartClassGenerator::new(class_name.clone().as_ref());
                generator.parse_value(v);
                self.nested_classes.extend(generator.get_result());
                class_name
            } else {
                self.parse_value(v)
            };
            self.fields
                .push((k.to_string().to_case(Case::Camel), type_name.to_string()));
        }
        "dynamic"
    }

    fn get_full_result(self) -> String {
        format!(
            r#"import 'package:json_annotation/json_annotation.dart';

part '{file_name}.g.dart';

{class}
"#,
            file_name = self.class_name.to_case(Case::Snake),
            class = self.get_result().join("\n\n"),
        )
    }
}
