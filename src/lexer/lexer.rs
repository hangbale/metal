use crate::lexer::token::*;
use crate::input::Code;
use crate::lexer::error::LexerError;
#[derive(Debug, PartialEq)]
enum State {
    Start,
    Number,
    String,
    Punctuator
}

pub struct Lexer<'a> {
    state: State,
    list: Vec<Token>,
    code: Code<'a>,
    cache: String,
    line: u64,
    column: u64
}

impl<'a> Lexer<'a> {
    pub fn new (code: &'a str) -> Self {
        Self {
            state: State::Start,
            code: Code::new(code),
            list: vec![],
            cache: String::new(),
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
    pub fn advance (&mut self) {
        while let Some(ch) = self.code.peek() {
            match ch {
                'a'..='z' | 'A'..='Z' | '_' | '$' => {
                    println!("in id");
                    self.handle_identifier();
                }
                _ => {
                    break;
                    // Err(LexerError::IllegalCharacter);
                }
            }
        }
        println!("{}", self.cache);
        // Ok()
    }
    pub fn handle_identifier (&mut self) -> Result<String, LexerError> {
        while let Some(c) = &self.code.peek() {
            let nt = *c;
            println!("{}", nt);
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
                println!("normal");
                self.accept(nt);
                self.code.next();
            }
        }
        Ok(self.token_finishup())
    }
    pub fn token_finishup (&mut self) -> String {
        return self.cache.clone();
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