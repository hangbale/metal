use crate::lexer::token::*;
use crate::input::Code;
use crate::lexer::error::LexerError;



pub struct Lexer<'a> {
    code: Code<'a>,
    cache: String,
    current: Option<Token>
}

impl<'a> Lexer<'a> {
    pub fn new (code: &'a str) -> Self {
        Self {
            code: Code::new(code),
            cache: String::new(),
            current: None
        }
    }
    pub fn advance (&mut self) -> Result<Option<Token>, LexerError> {
        while let Some(ch) = self.code.peek() {
            match ch {
                '=' => {
                    self.accept(ch);
                    self.set_token(TokenType::ASSIGN);
                    self.code.next();
                    break;
                }
                '"' | '\'' => {
                    self.code.next();
                    self.string_literal(ch)?;
                    break;
                }
                'a'..='z' | 'A'..='Z' | '_' | '$' => {
                    self.handle_identifier()?
                }
                _ => {
                    self.code.next();
                    break;
                }
            }
        }
        Ok(self.get_token())
    }
    pub fn get_token (&mut self) -> Option<Token> {
        self.current.take()
    }
    pub fn set_token (&mut self, tp: TokenType) {
        let v = self.cache.clone();
        let v_len = v.chars().count() as u64;
        let mut col: u64 = 0;
        // in string literal, v_len maybe larger than column_cursor
        if v_len > self.code.column_cursor {
            col = 0;
        } else {
            col = self.code.column_cursor - v_len;
        }
        self.current = Some(Token {
            value: v,
            category: tp,
            line: self.code.line_cursor,
            column: col
        });
        self.cache = String::new();
    }
    pub fn string_literal (&mut self, start: char) -> Result<(), LexerError> {
        loop {
            let nt = self.code.next();
            match nt {
                Some('\n') | Some('\r') => {
                    return Err(LexerError::InvalidString);
                }
                Some(c @ '"') | Some(c @ '\'') => {
                    if c == start {
                        self.set_token(TokenType::STRING_LITERAL);
                        break;
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
        Ok(())
    }
    pub fn handle_string_seq (&mut self) -> Result<(), LexerError> {
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
                                        self.code.next();
                                        break;
                                    } else {
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
        Ok(())
    }
    pub fn handle_identifier (&mut self) -> Result<(), LexerError>{
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
    pub fn token_finishup (&mut self) -> () {
        let kd = try_keyword(&self.cache);
        let value = self.cache.clone();
        let v_len = value.chars().count() as u64;
        self.current = Some(Token {
            value: value,
            category: kd,
            line: self.code.line_cursor,
            column: self.code.column_cursor - v_len
        });
        self.cache = String::new();
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