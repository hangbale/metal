pub enum LexerError {
    InvalidUnicodeSequence,
    UnicodeOverfow,
    UnicodeSurrogateCodePoint,
    InvalidHexNumber,
    IllegalCharacter
}