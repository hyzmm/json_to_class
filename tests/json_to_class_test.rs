use json_to_class::generators::dart_generator::DartClassGenerator;
use json_to_class::json_to_class;

#[test]
fn simple_type_test() {
    let json_string = r#"
    {
            "name": "WengXiang",
            "age": 27,
            "address": null,
            "married": true
        }"#;
    let generator = DartClassGenerator::new("Foo");
    let result = json_to_class(json_string, generator).unwrap();
    assert_eq!(
        result,
        r#"import 'package:json_annotation/json_annotation.dart';

part 'foo.g.dart';

@JsonSerializable()
class Foo {
    final dynamic address;
    final int age;
    final bool married;
    final String name;
    Foo({
        required this.address,
        required this.age,
        required this.married,
        required this.name,
    });

    factory Foo.fromJson(Map<String, dynamic> json) => _$FooFromJson(json);

    Map<String, dynamic> toJson() => _$FooToJson(this);
}"#
    );
}

#[test]
fn array_test() {
    let json_string = r#"{
        "hobbies": ["coding", "reading", "gaming"]
    }"#;
    let generator = DartClassGenerator::new("Foo");
    let result = json_to_class(json_string, generator).unwrap();
    assert_eq!(
        result,
        r#"import 'package:json_annotation/json_annotation.dart';

part 'foo.g.dart';

@JsonSerializable()
class Foo {
    final List<String> hobbies;
    Foo({
        required this.hobbies,
    });

    factory Foo.fromJson(Map<String, dynamic> json) => _$FooFromJson(json);

    Map<String, dynamic> toJson() => _$FooToJson(this);
}"#
    );
}

#[test]
fn array_with_different_types_elements_test() {
    let json_string = r#"{
        "data": [1, "gaming"]
    }"#;
    let generator = DartClassGenerator::new("Foo");
    let result = json_to_class(json_string, generator).unwrap();
    assert_eq!(
        result,
        r#"import 'package:json_annotation/json_annotation.dart';

part 'foo.g.dart';

@JsonSerializable()
class Foo {
    final List<dynamic> data;
    Foo({
        required this.data,
    });

    factory Foo.fromJson(Map<String, dynamic> json) => _$FooFromJson(json);

    Map<String, dynamic> toJson() => _$FooToJson(this);
}"#
    );
}
