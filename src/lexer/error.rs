#[derive(Debug)]
pub enum LexerError {
    InvalidUnicodeSequence,
    UnicodeOverfow,
    UnicodeSurrogateCodePoint,
    InvalidHexNumber,
    InvalidOctalNumber,
    IllegalCharacter,
    UnexpectedKeyword(String),
    InvalidString,
    InvalidOctalSeq,
    EOF,
    InvalidBinaryNumber,
    InvalidNumberSeq,
    UnexpectedToken(String)
}