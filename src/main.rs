mod lexer;
mod input;
extern crate unicode_segmentation;
fn main() {
    let mut lexer_ins = lexer::lexer::Lexer::new();
    lexer_ins.start();
}
