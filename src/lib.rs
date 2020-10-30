// use std::fs;

// pub use crate::{
//     _helpers,
//     lexer,
//     parser,
//     ast,
// };

pub mod _helpers;
pub mod lexer;
pub mod parser;
pub mod ast;



// fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
//     // let contents = fs::read_to_string("tests/2-happy-parser.odet")?;
//     // let contents = fs::read_to_string("tests/3-required-parser.odet")?;
//     // let contents = fs::read_to_string("tests/4-array-parser.odet")?;
//     // let contents = fs::read_to_string("tests/5-better-parser.odet")?;
//     // let contents = fs::read_to_string("tests/6-duplicate-field-name.odet")?;
//     // let contents = fs::read_to_string("tests/7-missing-type-parser.odet")?;
//     // let contents = fs::read_to_string("tests/8-comments-parser.odet")?;
//     let contents = fs::read_to_string("tests/9-annotation-parser.odet")?;

//     let mut l = lexer::Lexer::new(contents.as_str());
//     let tokens = l.run();
//     println!("Tokens:\n{}", tokens);
//     let mut p = parser::Parser::new(&tokens);
//     let output = p.parse();

//     match output {
//         Ok(o) => println!("Success:\n{}", o),
//         Err(e) => println!("Error: {:?}", e)
//     }

//     Ok(())
// }
