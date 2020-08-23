use std::fs;

mod lexer;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let contents = fs::read_to_string("tests/test1.odet")?;

    let mut l = lexer::Lexer::new(contents.as_str());
    let tokens = l.run();

    println!("{:?}", tokens);

    Ok(())
}
