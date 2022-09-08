use convert_case::Case;

use json_to_class::generators::{NamingRule, to_legal_case};
use json_to_class::generators::dart_generator::{DartClassGenerator};
use json_to_class::json_to_class;

#[test]
fn remove_illegal_characters_test() {
    assert_eq!(to_legal_case("a", Case::Pascal), "A");
    assert_eq!(to_legal_case("a", Case::Camel), "a");
    assert_eq!(to_legal_case("1a", Case::Pascal), "A");
    assert_eq!(to_legal_case("1a", Case::Camel), "a");
    assert_eq!(to_legal_case("1-a", Case::Pascal), "A");
    assert_eq!(to_legal_case("1-a", Case::Camel), "a");
    assert_eq!(to_legal_case("1a-a", Case::Pascal), "AA");
    assert_eq!(to_legal_case("1a-a", Case::Camel), "aA");
    assert_eq!(to_legal_case("1a-b", Case::Pascal), "AB");
    assert_eq!(to_legal_case("1a-b", Case::Camel), "aB");
    assert_eq!(to_legal_case("1a-b2", Case::Pascal), "AB2");
    assert_eq!(to_legal_case("1a-b2", Case::Camel), "aB2");
    assert_eq!(to_legal_case("1a-b2$", Case::Pascal), "AB2");
    assert_eq!(to_legal_case("1a-b2$", Case::Camel), "aB2");
    assert_eq!(to_legal_case("$meta", Case::Pascal), "Meta");
    assert_eq!(to_legal_case("$meta", Case::Camel), "meta");
    assert_eq!(to_legal_case("$meta-data", Case::Pascal), "MetaData");
    assert_eq!(to_legal_case("$meta-data", Case::Camel), "metaData");
    assert_eq!(to_legal_case("$a-b?", Case::Pascal), "AB");
    assert_eq!(to_legal_case("$a-b?", Case::Camel), "aB");
}

#[test]
fn simple_type_test() {
    let json_string = r#"
    {
            "name": "WengXiang",
            "age": 27,
            "address": null,
            "married": true
        }"#;
    let generator = DartClassGenerator::new("Foo", NamingRule::None);
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
    let generator = DartClassGenerator::new("Foo", NamingRule::None);
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
    let generator = DartClassGenerator::new("Foo", NamingRule::None);
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
    let generator = DartClassGenerator::new("Foo", NamingRule::None);
    let result = json_to_class(json_string, generator).unwrap();
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

#[test]
fn illegal_json_key() {
    let json_string = r#"{
        "$a-b?": "c"
    }"#;
    let generator = DartClassGenerator::new("Foo", NamingRule::None);
    let result = json_to_class(json_string, generator).unwrap();
    assert_eq!(
        result,
        r#"import 'package:json_annotation/json_annotation.dart';

part 'foo.g.dart';

@JsonSerializable()
class Foo {
    @JsonKey(name: "\$a-b?")
    final String aB;
    Foo({
        required this.aB,
    });

    factory Foo.fromJson(Map<String, dynamic> json) => _$FooFromJson(json);

    Map<String, dynamic> toJson() => _$FooToJson(this);
}
"#);
}


#[test]
fn json_key_with_numerical_prefix() {
    let json_string = r#"{
        "123ab": "c",
        "a1b": "d"
    }"#;
    let generator = DartClassGenerator::new("Foo", NamingRule::None);
    let result = json_to_class(json_string, generator).unwrap();
    assert_eq!(
        result,
        r#"import 'package:json_annotation/json_annotation.dart';

part 'foo.g.dart';

@JsonSerializable()
class Foo {
    @JsonKey(name: "123ab")
    final String ab;
    @JsonKey(name: "a1b")
    final String a1B;
    Foo({
        required this.ab,
        required this.a1B,
    });

    factory Foo.fromJson(Map<String, dynamic> json) => _$FooFromJson(json);

    Map<String, dynamic> toJson() => _$FooToJson(this);
}
"#);
}


#[test]
fn duplicated_json_keys_test() {
    let json_string = r#"{
        "a": "c",
        "a": "d"
    }"#;
    let generator = DartClassGenerator::new("Foo", NamingRule::None);
    let result = json_to_class(json_string, generator).unwrap();
    assert_eq!(
        result,
        r"import 'package:json_annotation/json_annotation.dart';

part 'foo.g.dart';

@JsonSerializable()
class Foo {
    final String a;
    Foo({
        required this.a,
    });

    factory Foo.fromJson(Map<String, dynamic> json) => _$FooFromJson(json);

    Map<String, dynamic> toJson() => _$FooToJson(this);
}
");
}


#[test]
fn duplicated_class_test() {
    let json_string = r#"{
        "a": { "b": "c" },
        "d": { "a": { "b": "c" } }
    }"#;
    let generator = DartClassGenerator::new("Foo", NamingRule::None);
    let result = json_to_class(json_string, generator).unwrap();
    assert_eq!(
        result,
        r"import 'package:json_annotation/json_annotation.dart';

part 'foo.g.dart';

@JsonSerializable()
class Foo {
    final A a;
    final D d;
    Foo({
        required this.a,
        required this.d,
    });

    factory Foo.fromJson(Map<String, dynamic> json) => _$FooFromJson(json);

    Map<String, dynamic> toJson() => _$FooToJson(this);
}

@JsonSerializable()
class A {
    final String b;
    A({
        required this.b,
    });

    factory A.fromJson(Map<String, dynamic> json) => _$AFromJson(json);

    Map<String, dynamic> toJson() => _$AToJson(this);
}

@JsonSerializable()
class D {
    final A a;
    D({
        required this.a,
    });

    factory D.fromJson(Map<String, dynamic> json) => _$DFromJson(json);

    Map<String, dynamic> toJson() => _$DToJson(this);
}
");
}


#[test]
fn same_class_name_with_different_content() {
    let json_string = r#"{
        "a": { "b": "c" },
        "d": { "a": { "e": "c" } }
    }"#;
    let generator = DartClassGenerator::new("Foo", NamingRule::None);
    let result = json_to_class(json_string, generator).unwrap();
    assert_eq!(
        result,
        r"import 'package:json_annotation/json_annotation.dart';

part 'foo.g.dart';

@JsonSerializable()
class Foo {
    final A a;
    final D d;
    Foo({
        required this.a,
        required this.d,
    });

    factory Foo.fromJson(Map<String, dynamic> json) => _$FooFromJson(json);

    Map<String, dynamic> toJson() => _$FooToJson(this);
}

@JsonSerializable()
class A {
    final String b;
    A({
        required this.b,
    });

    factory A.fromJson(Map<String, dynamic> json) => _$AFromJson(json);

    Map<String, dynamic> toJson() => _$AToJson(this);
}

@JsonSerializable()
class D {
    final A1 a;
    D({
        required this.a,
    });

    factory D.fromJson(Map<String, dynamic> json) => _$DFromJson(json);

    Map<String, dynamic> toJson() => _$DToJson(this);
}

@JsonSerializable()
class A1 {
    final String e;
    A1({
        required this.e,
    });

    factory A1.fromJson(Map<String, dynamic> json) => _$A1FromJson(json);

    Map<String, dynamic> toJson() => _$A1ToJson(this);
}
");
}