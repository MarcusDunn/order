use pest::Parser;

use super::*;

#[test]
fn test_number_literal1() {
    OrderParser::parse(Rule::number, "1 ").unwrap();
}

#[test]
fn test_number_literal2() {
    OrderParser::parse(Rule::number, "-123_122").unwrap();
}

#[test]
fn test_string_literal1() {
    OrderParser::parse(Rule::string, "\"hello world\"").unwrap();
}

#[test]
fn test_literal1() {
    OrderParser::parse(Rule::literal, "\"hello world\"").unwrap();
}

#[test]
fn test_literal2() {
    OrderParser::parse(Rule::literal, "-123_122").unwrap();
}

#[test]
fn test_literal3() {
    OrderParser::parse(Rule::literal, "1").unwrap();
}

#[test]
fn test_identifier1() {
    OrderParser::parse(Rule::identifier, "myVar").unwrap();
}

#[test]
fn test_identifier2() {
    OrderParser::parse(Rule::identifier, "addOne").unwrap();
}

#[test]
fn test_identifier3() {
    let mut parsed = OrderParser::parse(Rule::identifier, "addOne  bida\tokf\n\tsdlh\n").unwrap();
    assert_eq!(parsed.next().unwrap().as_span().as_str(), "addOne")
}

#[test]
fn test_signature1() {
    OrderParser::parse(Rule::typeSignature, "Int").unwrap();
}

#[test]
fn test_signature2() {
    OrderParser::parse(Rule::typeSignature, "Int -> Int").unwrap();
}

#[test]
fn test_signature3() {
    OrderParser::parse(Rule::typeSignature, "Int -> Int -> Int").unwrap();
}

#[test]
fn test_signature4() {
    OrderParser::parse(Rule::typeSignature, "Int -> Int -> Int  ").unwrap();
}

#[test]
fn test_declaration1() {
    OrderParser::parse(Rule::declaration, "one :: Int\n").unwrap();
}

#[test]
fn test_declaration2() {
    OrderParser::parse(Rule::declaration, "one :: Int \n").unwrap();
}

#[test]
fn test_declaration3() {
    OrderParser::parse(Rule::declaration, "one :: Int -> Int\n").unwrap();
}

#[test]
fn test_pattern1() {
    OrderParser::parse(Rule::pattern, "a b").unwrap();
}

#[test]
fn test_action1() {
    OrderParser::parse(Rule::action, "plus a b").unwrap();
}

#[test]
fn test_resolver() {
    OrderParser::parse(Rule::resolver, "id a = a\n").unwrap();
}

#[test]
fn test_function_definition1() {
    OrderParser::parse(Rule::functionDefinition, "id :: Int -> Int\nid a = a\n\n").unwrap();
}

#[test]
fn test_function_definition2() {
    OrderParser::parse(Rule::functionDefinition, "one :: Int\none = 1\n\n").unwrap();
}
