use regex::Regex;
use unicode_xid::UnicodeXID;
use crate::lexer::error::LexerError;
#[derive(Debug)]
pub enum TokenType {
    // keywords
    BREAK,
    SWITCH,
    CASE,
    CONTINUE,
    FOR,
    LET,
    VAR,
    FUNCTION,
    THIS,
    CATCH,
    TRY,
    FINALLY,
    THROW,
    WITH,
    WHILE,
    DO,
    IF,
    ELSE,
    RETURN,
    DEFAULT,
    DELETE,
    TYPEOF,
    INSTANCEOF,
    IN,
    NEW,
    // literal
    FALSE_LITERAL,
    TRUE_LITERAL,
    NULL_LITERAL,
    UNDEFINED_LITERAL,
    NUMERIC_LITERAL,
    STRING_LITERAL,
    NUMERIC_LITERAL_BINARY,
    NUMERIC_LITERAL_OCTAL,
    NUMERIC_LITERAL_DECIMAL,
    NUMERIC_LITERAL_HEX,
    // punctuator
    SEMICOLON,
    PERIOD, // .
    LBRACK, // [
    RBRACK, // ]
    LPAREN, // (
    RPAREN, // )
    LBRACE, // {
    RBRACE, // }
    COLON, // :
    COMMA, // ,
    NOT, // !
    INC, // ++
    DEC, // --
    EQ, // ==
    EQ_STRICT, // ===
    NE_STRICT, // !==
    NE, // !=
    LT, // <
    GT, // >
    LTE, // <=
    GTE, // >=
    ADD, // +
    SUB, // -
    AND, // &&
    OR, // ||
    ASSIGN, // =
    UNHANDLED,
    Identifier
}


pub const Number_Regex: &str = r"\d";
pub const Punctuators_Regex: &str = r"\+|\-|=";
pub const Emoji_Regex: &str = r"\p{Emoji}";

// special unicode chars
// ZERO WIDTH NON-JOINER, in identifiers
pub const ZWNJ: char = '\u{200c}';
// ZERO WIDTH JOINER, in identifiers
pub const ZWJ: char = '\u{200d}';
// ZERO WIDTH NO-BREAK SPACE, a whitespace
pub const ZWNBSP: char = '\u{feff}';

// whitespace
pub const TAB: char = '\u{9}';
pub const SPACE: char = '\u{20}';
pub const NBSP: char = '\u{a0}';
pub const FF: char = '\u{c}';

// line ending
pub const LF: char = '\u{a}';
pub const CR: char = '\u{d}';
pub const LS: char = '\u{2028}';
pub const PS: char = '\u{2029}';

#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub number: Option<f64>,
    pub category: TokenType,
    pub line: u64,
    pub column: u64
}

pub fn is_numeric_token (tp: TokenType) -> bool {
    match tp {
        TokenType::NUMERIC_LITERAL_BINARY | TokenType::NUMERIC_LITERAL_DECIMAL |
        TokenType::NUMERIC_LITERAL_OCTAL | TokenType::NUMERIC_LITERAL_HEX => true,
        _ => false
    }
}

pub fn is_alphabetic (s: char) -> bool {
    return s.is_alphabetic(); 
}

pub fn is_numeric (s: &str) -> bool {
    let st = s.to_string();
    let re = Regex::new(Number_Regex);
    if let Ok(reg) = re {
        return reg.is_match(&st);
    } else {
        false
    }
}
pub fn is_punctuator (s: &str) -> bool {
    let st = s.to_string();
    let re = Regex::new(Punctuators_Regex);
    if let Ok(reg) = re {
        return reg.is_match(&st);
    } else {
        false
    }
}
pub fn get_punctuator_type (s: &str) -> TokenType {
    match s {
        "+" => TokenType::ADD,
        "-" => TokenType::SUB,
        "=" => TokenType::ASSIGN,
        "(" => TokenType::LPAREN,
        ")" => TokenType::RPAREN,
        _ => TokenType::UNHANDLED,
    }
}
pub fn is_string_literal_start (s: char) -> bool {
    return  s == '\'' || s == '"';
}

pub fn is_identifier_start (s: char) -> bool {
    if s.is_ascii() {
        return s == '_' || s == '$' || s.is_ascii_alphabetic()
    } else {
        return UnicodeXID::is_xid_start(s);
    }
}
pub fn is_identifier_continue(c: char) -> bool {
    if c.is_ascii() {
        c == '$' || c == '_' || c.is_ascii_alphanumeric()
    } else {
        UnicodeXID::is_xid_continue(c) || c == ZWNJ || c == ZWJ
    }
}

pub fn is_unicode_seq_start (s: char) -> bool {
    return s == '\\';
}
// an unicode sequence identifier
// let \u0061 = 'foobar'; let \u{0061} = 'foobar';let \u{0061}name = 'foobar'
// read 4 chars next '\u' or '\u{' as a decimal number, then translate them to a char and check its validity

pub fn hex_char_to_digit (s: char) -> Result<u32, LexerError> {
    match s {
        c @ '0'..='9' => Ok(c as u32 - '0' as u32),
        c @ 'a'..='f' => Ok(10 + (c as u32 - 'a' as u32)),
        c @ 'A'..='F' => Ok(10 + (c as u32 - 'A' as u32)),
        _ => Err(LexerError::InvalidHexNumber)
    }
}

pub fn digit_to_char (s: u32) -> Result<char, LexerError> {
    if s > 0x10FFFF {
        return Err(LexerError::UnicodeOverfow);
    }
    if s >= 0xd800 && s <= 0xdfff {
        return Err(LexerError::UnicodeSurrogateCodePoint);
    }
    char::try_from(s).map_err(|e| LexerError::InvalidUnicodeSequence)
}
pub fn try_keyword (s: &str) -> TokenType {
    match s {
        "break" => TokenType::BREAK,
        "switch" => TokenType::SWITCH,
        "case" => TokenType::CASE,
        "continue" => TokenType::CONTINUE,
        "for" => TokenType::FOR,
        "let" => TokenType::LET,
        "var" => TokenType::VAR,
        "function" => TokenType::FUNCTION,
        "this" => TokenType::THIS,
        "catch" => TokenType::CATCH,
        "try" => TokenType::TRY,
        "finally" => TokenType::FINALLY,
        "throw" => TokenType::THROW,
        "with" => TokenType::WITH,
        "while" => TokenType::WHILE,
        "do" => TokenType::DO,
        "if" => TokenType::IF,
        "else" => TokenType::ELSE,
        "return" => TokenType::RETURN,
        "default" => TokenType::DEFAULT,
        "delete" => TokenType::DELETE,
        "typeof" => TokenType::TYPEOF,
        "instanceof" => TokenType::INSTANCEOF,
        "new" => TokenType::NEW,
        "in" => TokenType::IN,
        "false" => TokenType::FALSE_LITERAL,
        "true" => TokenType::TRUE_LITERAL,
        "null" => TokenType::NULL_LITERAL,
        "undefined" => TokenType::UNDEFINED_LITERAL,
        _ => TokenType::Identifier
    }
}

