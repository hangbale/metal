use crate::lexer::token::*;
use crate::input::Code;
use crate::lexer::error::LexerError;



pub struct Lexer<'a> {
    list: Vec<Token>,
    code: Code<'a>,
    cache: String,
    current: Option<Token>,
    line: u64,
    column: u64
}

impl<'a> Lexer<'a> {
    pub fn new (code: &'a str) -> Self {
        Self {
            code: Code::new(code),
            list: vec![],
            cache: String::new(),
            current: None,
            line: 0,
            column: 0
        }
    }
    pub fn print (&mut self) {
        for _ in 0..10 {
            let t = self.code.peek();
            println!("{:?}", t);
        }
    }
    pub fn advance (&mut self) -> Result<Option<Token>, LexerError> {
        while let Some(ch) = self.code.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' | '_' | '$' => {
                    self.handle_identifier()?
                }
                _ => {
                    self.code.next();
                    break;
                    // Err(LexerError::IllegalCharacter);
                }
            }
        }
        Ok(self.get_token())
    }
    pub fn get_token (&mut self) -> Option<Token> {
        self.current.take()
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
        self.current = Some(Token {
            value: self.cache.clone(),
            category: kd,
            line: self.code.line_cursor,
            column: self.code.column_cursor
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