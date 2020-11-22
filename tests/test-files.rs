use std::fs;
use std::time::{Instant};

use odetto::{lexer, parser, ast, _helpers::ParseError};

#[test]
fn happy_parser() {
  let result = test_file("tests/odet-files/2-happy-parser.odet");
  assert!(result.is_ok(), "all good!");
  println!("{}", result.unwrap());
}

#[test]
fn required_parser() {
  let result = test_file("tests/odet-files/3-required-parser.odet");
  assert!(result.is_ok(), "all good!");
  println!("{}", result.unwrap());
}

#[test]
fn array_parser() {
  let result = test_file("tests/odet-files/4-array-parser.odet");
  assert!(result.is_ok(), "all good!");
  println!("{}", result.unwrap());
}

#[test]
fn better_parser() {
  let result = test_file("tests/odet-files/5-better-parser.odet");
  assert!(result.is_ok(), "all good!");
  println!("{}", result.unwrap());
}

#[test]
fn duplicate_field_parser() {
  let result = test_file("tests/odet-files/6-duplicate-field-name-parser.odet");
  assert!(result.is_err(), "errored correctly!");
  println!("{}", result.unwrap_err());
}

#[test]
fn missing_type_parser() {
  let result = test_file("tests/odet-files/7-missing-type-parser.odet");
  assert!(result.is_err(), "errored correctly!");
  println!("{}", result.unwrap_err());
}

#[test]
fn comments_parser() {
  let result = test_file("tests/odet-files/8-comments-parser.odet");
  assert!(result.is_ok(), "all good!");
  println!("{}", result.unwrap());
}

#[test]
fn annotation_parser() {
  let result = test_file("tests/odet-files/9-annotation-parser.odet");
  assert!(result.is_ok(), "all good!");
  println!("{}", result.unwrap());
}

fn test_file(file_name: &str) -> Result<ast::Root, ParseError> {
    let start = Instant::now();
    let contents = fs::read_to_string(file_name).unwrap();
    let mut l = lexer::Lexer::new(contents.as_str());
    let tokens = l.run();
    println!("Tokens:\n{}", tokens);
    let mut p = parser::Parser::new(&tokens);

    let result = p.parse();
    println!("{} took {:?}", file_name, start.elapsed());

    return result;
}