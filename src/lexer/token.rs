use regex::Regex;

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
#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub category: TokenType,
    pub line: u64,
    pub column: u64
}

pub fn is_alphabetic (s: char) -> bool {
    return s.is_alphabetic();
    // let chars: Vec<char> = s.to_string().chars().collect();
    // if chars.len() == 1 {
    //     let f = chars[0];
        
    // } else {
    //     println!("not a single char");
    //     let re = Regex::new(Emoji_Regex);
    //     if let Ok(reg) = re {
    //         return reg.is_match(&s);
    //     } else {
    //         false
    //     }
    // }
    
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
pub fn get_strings_type (s: &str) -> TokenType {
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

