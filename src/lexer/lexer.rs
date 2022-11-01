use super::token::*;
use crate::input::Code;
use super::error::LexerError;
use super::util::*;


pub struct Lexer<'a> {
    code: Code<'a>,
    cache: String
}

impl<'a> Lexer<'a> {
    pub fn new (code: &'a str) -> Self {
        Self {
            code: Code::new(code),
            cache: String::new()
        }
    }
    pub fn advance (&mut self) -> Result<Token, LexerError> {
        while let Some(ch) = self.code.peek() {
            match ch {
                LF | CR | PS | LS | TAB | FF | NBSP | SPACE => {
                    self.code.next();
                    continue;
                }
                '=' => {
                    return Ok(self.set_one_char_token(ch, TokenType::ASSIGN));
                }
                ':' => {
                    return Ok(self.set_one_char_token(ch, TokenType::COLON));
                }
                '[' => {
                    return Ok(self.set_one_char_token(ch, TokenType::LBRACK));
                }
                ']' => {
                    return Ok(self.set_one_char_token(ch, TokenType::RBRACK));
                }
                '(' => {
                    return Ok(self.set_one_char_token(ch, TokenType::LPAREN));
                }
                ')' => {
                    return Ok(self.set_one_char_token(ch, TokenType::RPAREN));
                }
                '{' => {
                    return Ok(self.set_one_char_token(ch, TokenType::LBRACE));
                }
                '}' => {
                    return Ok(self.set_one_char_token(ch, TokenType::RBRACE));
                }
                ',' => {
                    return Ok(self.set_one_char_token(ch, TokenType::COMMA));
                }
                '"' | '\'' => {
                    self.set_column_start();
                    self.code.next();
                    return self.string_literal(ch);
                }
                'a'..='z' | 'A'..='Z' | '_' | '$' => {
                    self.set_column_start();
                    return self.handle_identifier();
                }
                ';' => {
                    return Ok(self.set_one_char_token(ch, TokenType::SEMICOLON));
                }
                '0' => {
                    return self.handle_number_start_with_zero();
                }
                '1'..='9' => {
                    return self.handle_decimal_numeric(false);
                }
                _ => {
                    self.code.next();
                    break;
                }
            }
        }
        Err(LexerError::EOF)
    }
    fn set_column_start(&mut self) {
        self.code.column_start = self.code.column_cursor;
    }
    fn set_one_char_token(&mut self, ch: char, tp: TokenType) -> Token {
        self.set_column_start();
        self.code.next();
        Token {
            value: ch.to_string(),
            category: tp,
            line: self.code.line_cursor,
            column: self.code.column_start,
            number: None
        }
    }
    // pub fn get_token (&mut self) -> Option<Token> {
    //     self.current.take()
    // }
    pub fn set_token (&mut self, tp: TokenType) -> Token {
        let v = self.cache.clone();
        let num = parse_numeric(&v, &tp).ok();
        self.cache = String::new();
        Token {
            value: v,
            category: tp,
            line: self.code.line_cursor,
            column: self.code.column_start,
            number: num
        }
    }
    fn handle_decimal_numeric (&mut self, radix_prefix: bool)-> Result<Token, LexerError>{
        let mut has_radix = false;
        while let Some(nt) = self.code.peek() {
            match nt {
                '0'..='9' => {
                    self.accept(nt);
                    self.code.next();
                }
                '.' => {
                    if radix_prefix || has_radix {
                        return Err(LexerError::UnexpectedToken(" . ".to_string()))
                    } else {
                        has_radix = true;
                        self.accept('.');
                        self.code.next();
                    }
                }
                '_' => {
                    self.code.next();
                }
                _ => {
                    break;
                }
            }
        }
        Ok(self.set_token(TokenType::NUMERIC_LITERAL))
    }
    fn handle_number_start_with_zero(&mut self) -> Result<Token, LexerError>{
        self.code.next();
        match self.code.peek() {
            Some('O') | Some('o') => {
                self.code.next();
                self.octal_number()?;
                return Ok(self.set_token(TokenType::NUMERIC_LITERAL));
            }
            Some('0'..='9') => {
                return self.octal_or_decimal_number();
            }
            Some('b') | Some('B') => {
                self.code.next();
                self.binary_number()?;
                return Ok(self.set_token(TokenType::NUMERIC_LITERAL));
            }
            Some('x') | Some('X') => {
                self.code.next();
                self.hex_number()?;
                return Ok(self.set_token(TokenType::NUMERIC_LITERAL));
            }
            _ => {
                Err(LexerError::InvalidNumberSeq)
            }
        }
    }
    fn octal_or_decimal_number(&mut self) -> Result<Token, LexerError>{
        let mut is_decimal = false;
        loop {
            let pc = self.code.peek();
            match pc {
                Some(c) => {
                    match c {
                        '0'..='7' => {
                            self.code.next();
                            self.accept(c);
                        }
                        '8'..='9' => {
                            self.code.next();
                            is_decimal = true;
                            self.accept(c);
                        }
                        '_' => {
                            self.code.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        self.check_after_numeric()?;
        if is_decimal {
            return Ok(self.set_token(TokenType::NUMERIC_LITERAL));
        } else {
            return Ok(self.set_token(TokenType::NUMERIC_LITERAL));
        }
    }
    fn octal_number(&mut self) -> Result<(), LexerError>{
        loop {
            let pc = self.code.peek();
            match pc {
                Some(c) => {
                    match c {
                        '0'..='7' => {
                            self.code.next();
                            self.accept(c);
                        }
                        '_' => {
                            self.code.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        self.check_after_numeric()
    }
    fn binary_number(&mut self) -> Result<(), LexerError>{
        loop {
            let pc = self.code.peek();
            match pc {
                Some(c) => {
                    match c {
                        '0'..='1' => {
                            self.code.next();
                            self.accept(c);
                        }
                        '_' => {
                            self.code.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        self.check_after_numeric()
    }
    fn hex_number(&mut self) -> Result<(), LexerError>{
        loop {
            let pc = self.code.peek();
            match pc {
                Some(c) => {
                    match c {
                        '0'..='9' | 'a'..='f' | 'A'..='F' => {
                            self.code.next();
                            self.accept(c);
                        }
                        '_' => {
                            self.code.next();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                None => {
                    break;
                }
            }
        }
        self.check_after_numeric()
    }
    fn check_after_numeric(&mut self) -> Result<(), LexerError> {
        let nc = self.code.peek();
        if let Some(c) = nc {
            if is_identifier_start(c) || c.is_digit(10) {
                return Err(LexerError::UnexpectedToken(c.to_string()));
            }
        }
        Ok(())
    }
    pub fn string_literal (&mut self, start: char) -> Result<Token, LexerError> {
        loop {
            let nt = self.code.next();
            match nt {
                Some('\n') | Some('\r') => {
                    return Err(LexerError::InvalidString);
                }
                Some(c @ '"') | Some(c @ '\'') => {
                    if c == start {
                        return Ok(self.set_token(TokenType::STRING_LITERAL));
                    } else {
                        self.accept(c);
                    }
                }
                Some('\\') => {
                    self.handle_string_seq()?;
                }
                Some(c) => {
                    self.accept(c);
                }
                None => break
            }
        }
        return Ok(self.set_token(TokenType::STRING_LITERAL));
    }
    pub fn handle_string_seq (&mut self) -> Result<Token, LexerError> {
        match self.code.next() {
            None => {
                return Err(LexerError::InvalidString);
            }
            Some(c) => match c {
                // special escape char
                'b' => {
                    self.accept('\u{8}');
                }
                't' => {
                    self.accept('\u{9}');
                }
                'n' => {
                    self.accept('\u{a}');
                }
                'v' => {
                    self.accept('\u{b}');
                }
                'f' => {
                    self.accept('\u{c}');
                }
                'r' => {
                    self.accept('\u{d}');
                }
                '\'' | '\"' | '\\' => {
                    self.accept(c);
                }
                // \xXX, must be followed by 2 hex char
                'x' => {
                    let mut v: u32 = 0;
                    for _ in 0..2 {
                        let pn = self.code.peek();
                        if let Some(n) = pn {
                            let dtr = hex_char_to_digit(n);
                            match dtr {
                                Ok(d) => {
                                    v = v << 4 | d;
                                    self.code.next();
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        } else {
                            return Err(LexerError::InvalidOctalSeq);
                        }
                    }
                    let t = digit_to_char(v)?;
                    self.accept(t);
                },
                // \xxx, 0-3 octal number
                '0'..='7' => {
                    let mut v: u32 = 0;
                    let mut tmp: u32 = 0;
                    tmp = tmp << 3 | (c as u32 - 48);
                    for _ in 0..2 {
                        let pn = self.code.peek();
                        if let Some(n) = pn {
                            match n {
                                '0'..='7' => {
                                    let to_num = n as u32 - 48;
                                    tmp = tmp << 3 | to_num;
                                    if tmp > 255 {
                                        break;
                                    } else {
                                        self.code.next();
                                        v = tmp;
                                    }
                                },
                                _ => {
                                    break;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    let c = char::try_from(v)
                    .map_err(|_e| LexerError::InvalidOctalSeq);
                    match c {
                        Ok(cc) => {
                            self.accept(cc);
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                // \uxxxx or \u{xxxx}
                'u' => {
                    let pn = self.code.peek();
                    match pn {
                        Some('{') => {
                            self.code.next();
                            let c = self.try_code_point();
                            match c {
                                Err(e) => {
                                    return Err(e);
                                }
                                Ok(c) => {
                                    self.accept(c);
                                    self.code.next();
                                }
                            }
                        },
                        Some(_) => {
                            let c = self.try_four_hex_num();
                            match c {
                                Err(e) => {
                                    return Err(e);
                                }
                                Ok(c) => {
                                    self.accept(c);
                                }
                            }
                        }
                        None => {
                            return Err(LexerError::InvalidUnicodeSequence);
                        }
                    }
                }
                other => {
                    self.accept(other);
                }

            }
        }
        Ok(self.set_token(TokenType::STRING_LITERAL))
    }
    pub fn handle_identifier (&mut self) -> Result<Token, LexerError>{
        while let Some(c) = &self.code.peek() {
            let nt = *c;
            if !is_identifier_continue(nt) {
                if nt == '\\' {
                    let v = self.handle_unicode_seq()?;
                    if !is_identifier_continue(v) {
                        return Err(LexerError::InvalidUnicodeSequence);
                    }
                    continue;
                } else {
                    break;
                }
            } else {
                self.accept(nt);
                self.code.next();
            }
        }
        Ok(self.token_finishup())
    }
    pub fn token_finishup (&mut self) -> Token {
        let kd = try_keyword(&self.cache);
        return self.set_token(kd);
    }
    pub fn handle_unicode_seq (&mut self) -> Result<char, LexerError> {
        if let Some(next) = self.code.next() {
            if next == 'u' {
                match self.code.peek() {
                    Some('{') => {
                        return self.try_code_point();
                    }
                    Some(_) => {
                        return self.try_four_hex_num();
                    }
                    None => {
                        return Err(LexerError::InvalidUnicodeSequence);
                    }
                }
            } else {
                Err(LexerError::InvalidUnicodeSequence)
            }
        } else {
            Err(LexerError::InvalidUnicodeSequence)
        }
    }
    pub fn try_four_hex_num (&mut self) -> Result<char, LexerError>{
        let mut v: u32 = 0;
        for _ in 0..4 {
            let nt = self.code.next();
            if let Some(ch) = nt {
                let dg = hex_char_to_digit(ch)?;
                v = v << 4 | dg;
            } else {
                return Err(LexerError::InvalidUnicodeSequence);
            }
        }
        digit_to_char(v)
    }
    pub fn try_code_point (&mut self) -> Result<char, LexerError> {
        let mut v: u32 = 0;
        loop {
            let n = match self.code.peek() {
                Some(c @ '0'..='9') => c as u32 - '0' as u32,
                Some(c @ 'a'..='f') => 10 + (c as u32 - 'a' as u32),
                Some(c @ 'A'..='F') => 10 + (c as u32 - 'A' as u32),
                None => {
                    return Err(LexerError::InvalidUnicodeSequence)
                },
                Some(_) => break,
            };
            self.code.next();
            v = v << 4 | n;
        }
        digit_to_char(v)
    }
    pub fn accept (&mut self, tk: char) {
        self.cache.push(tk);
    }
}