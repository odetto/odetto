use std::fs;

mod lexer;
mod parser;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let contents = fs::read_to_string("tests/test1.odet")?;

    let mut l = lexer::Lexer::new(contents.as_str());
    let tokens = l.run();
    println!("{}", tokens);
    let mut p = parser::Parser::new(&tokens);
    let output = p.parse();

    match output {
        Ok(o) => println!("Success: {:?}", o),
        Err(e) => println!("Error: {:?}", e)
    }

    Ok(())
}
