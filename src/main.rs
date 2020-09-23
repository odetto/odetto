use std::fs;

mod _helpers;
mod lexer;
mod parser;
mod ast;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // let contents = fs::read_to_string("tests/2-happy-parser.odet")?;
    let contents = fs::read_to_string("tests/3-required-parser.odet")?;
    // let contents = fs::read_to_string("tests/6-duplicate-field-name.odet")?;

    let mut l = lexer::Lexer::new(contents.as_str());
    let tokens = l.run();
    println!("Tokens:\n{}", tokens);
    let mut p = parser::Parser::new(&tokens);
    let output = p.parse();

    match output {
        Ok(o) => println!("Success:\n{}", o),
        Err(e) => println!("Error: {:?}", e)
    }

    Ok(())
}
