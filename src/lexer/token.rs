use regex::Regex;
use unicode_xid::UnicodeXID;
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
    NUMBER_LITERAL,
    STRING_LITERAL,
    // punctuator
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
    UNHANDLED
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


#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub category: TokenType,
    pub line: u64,
    pub column: u64
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
// an unicode sequence identifier
// let \u0061 = 'foobar'; let \u{0061} = 'foobar';let \u{0061}name = 'foobar'
// read 4 chars next '\u' or '\u{' as a decimal number, then translate them to a char and check its validity
pub fn try_unicode_sequene_identifier() -> bool {
    let four_d = four_hex_to_digit();
    let ch_r = digit_to_char(four_d);
    if let Ok(ch) = ch_r {
        is_identifier_start(ch)
    } else {
        false
    }
}
fn hex_char_to_digit (s: char) -> Result<u32, String> {
    match s {
        c @ '0'..='9' => Ok(c as u32 - '0' as u32),
        c @ 'a'..='f' => Ok(10 + (c as u32 - 'a' as u32)),
        c @ 'A'..='F' => Ok(10 + (c as u32 - 'A' as u32)),
        _ => Err("parse error".to_string()) // TODO: a custom parse error
    }
}
pub fn four_hex_to_digit() -> u32 {
    let mut v:u32 = 0;
    for i in 0..4 {
        let d = hex_char_to_digit(); // todo
        if let Ok(dv) = d {
            v = (v << 4) | d;
        }
    }
    v
}
fn digit_to_char (s: u32) -> Result<char, String> {
    if s > 0xd800 && s < 0xdfff {
        return Err("surrogate code points".to_string());
    }
    return char::try_from(s).map_err(|e| String::from("invalid unicode sequence"));
}
pub fn get_type_from_string (s: &str) -> TokenType {
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
        _ => TokenType::STRING_LITERAL
    }
}

