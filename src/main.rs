use std::env;
use std::fs;

mod lexer;
mod input;
fn main() {
    let arg = env::args().nth(1).expect("require a js file");
    let content = fs::read_to_string(arg).expect("read file failed");
    let mut lexer_ins = lexer::lexer::Lexer::new(&content);

    // use advance method to get next token
    let t = lexer_ins.advance();
    println!("{:?}", t);
}
