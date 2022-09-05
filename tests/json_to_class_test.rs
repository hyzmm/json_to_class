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
}
"#
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
}
"#
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
}
"#
    );
}

#[test]
fn nested_object_test() {
    let json_string = r#"{
        "a": {
            "b": "c",
            "d": { "h": 1 }
        },
        "e": {
            "f": null
        },
        "g": 2
    }"#;
    let generator = DartClassGenerator::new("Foo");
    let result = json_to_class(json_string, generator).unwrap();
    println!("{}", result);
    assert_eq!(
        result,
        r#"import 'package:json_annotation/json_annotation.dart';

part 'foo.g.dart';

@JsonSerializable()
class Foo {
    final A a;
    final E e;
    final int g;
    Foo({
        required this.a,
        required this.e,
        required this.g,
    });

    factory Foo.fromJson(Map<String, dynamic> json) => _$FooFromJson(json);

    Map<String, dynamic> toJson() => _$FooToJson(this);
}

@JsonSerializable()
class A {
    final String b;
    final D d;
    A({
        required this.b,
        required this.d,
    });

    factory A.fromJson(Map<String, dynamic> json) => _$AFromJson(json);

    Map<String, dynamic> toJson() => _$AToJson(this);
}

@JsonSerializable()
class D {
    final int h;
    D({
        required this.h,
    });

    factory D.fromJson(Map<String, dynamic> json) => _$DFromJson(json);

    Map<String, dynamic> toJson() => _$DToJson(this);
}

@JsonSerializable()
class E {
    final dynamic f;
    E({
        required this.f,
    });

    factory E.fromJson(Map<String, dynamic> json) => _$EFromJson(json);

    Map<String, dynamic> toJson() => _$EToJson(this);
}
"#
    );
}
