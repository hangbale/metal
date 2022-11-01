use std::env;
use std::fs;

mod lexer;
mod input;
mod parser;
mod ast;
fn main() {
    let arg = env::args().nth(1).expect("require a js file");
    let content = fs::read_to_string(arg).expect("read file failed");
    let mut lexer_ins = lexer::lexer::Lexer::new(&content);
    let mut pas = parser::Parser::new(lexer_ins);
    pas.parse();
    // use advance method to get next token
    // for _ in 0..20 {
    //     let t = lexer_ins.advance();
    //     println!("{:?}", t);
    // }

}
