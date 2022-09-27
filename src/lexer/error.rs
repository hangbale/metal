#[derive(Debug)]
pub enum LexerError {
    InvalidUnicodeSequence,
    UnicodeOverfow,
    UnicodeSurrogateCodePoint,
    InvalidHexNumber,
    IllegalCharacter,
    UnexpectedKeyword(String),
}