use crate::lexer::error::{LexerError};
#[derive(Debug)]
pub enum ParseError {
    LexerError(LexerError),
    AlreadyDeclared(String),
    UnexpectedToken(String)
}