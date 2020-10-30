use std::fs;

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
    let contents = fs::read_to_string(file_name).unwrap();
    // let contents = fs::read_to_string("tests/3-required-parser.odet")?;
    // let contents = fs::read_to_string("tests/4-array-parser.odet")?;
    // let contents = fs::read_to_string("tests/5-better-parser.odet")?;
    // let contents = fs::read_to_string("tests/6-duplicate-field-name.odet")?;
    // let contents = fs::read_to_string("tests/7-missing-type-parser.odet")?;
    // let contents = fs::read_to_string("tests/8-comments-parser.odet")?;
    // let contents = fs::read_to_string("tests/9-annotation-parser.odet")?;

    let mut l = lexer::Lexer::new(contents.as_str());
    let tokens = l.run();
    println!("Tokens:\n{}", tokens);
    let mut p = parser::Parser::new(&tokens);

    let result = p.parse();

    return result;

    // match output {
    //     Ok(o) => println!("Success:\n{}", o),
    //     Err(e) => println!("Error: {:?}", e)
    // }

    // Ok(())
}