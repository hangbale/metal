use crate::lexer::token::{self, Token, TokenType};
use crate::input::Code;
#[derive(Debug, PartialEq)]
enum State {
    Start,
    Number,
    String,
    Punctuator
}

pub struct Lexer {
    state: State,
    list: Vec<Token>,
    code: Code,
    cache: String,
    line: u64,
    column: u64
}

impl Lexer {
    pub fn new () -> Self {
        Self {
            state: State::Start,
            code: Code::new(),
            list: vec![],
            cache: String::from(""),
            line: 0,
            column: 0
        }
    }
    pub fn set_state (&mut self, state: State) {
        self.state = state;
    }
    pub fn forwrad (&mut self) {
        match self.state {
            State::String => {
                while let Some(char) = self.code.peek() {
                    let chars: Vec<char> = char.chars().collect();
                    if chars.len() > 1 || token::is_alphabetic(chars[0]) {
                        self.cache.push_str(&char);
                        let char_o = self.code.next();
                        if let Some(char_r) = char_o {
                            if self.line == 0 {
                                self.line = char_r.line;
                                self.column = char_r.column;
                            }
                        }
                    } else {
                        self.state = State::Start;
                        let t_type = token::get_strings_type(&self.cache);
                        self.accept(Token {
                            value: self.cache.clone(),
                            category: t_type,
                            line: self.line,
                            column: self.column
                        });
                        self.cache = String::from("");
                        self.line = 0;
                        self.column = 0;
                        break;
                    }
                }
            }
            State::Number => {
                while let Some(char) = self.code.peek() {
                    if token::is_numeric(&char) {
                        self.cache.push_str(&char);
                        let char_o = self.code.next();
                        if let Some(char_r) = char_o {
                            if self.line == 0 {
                                self.line = char_r.line;
                                self.column = char_r.column;
                            }
                        }
                    } else {
                        self.state = State::Start;
                        self.accept(Token {
                            value: self.cache.clone(),
                            category: TokenType::NUMBER_LITERAL,
                            line: self.line,
                            column: self.column
                        });
                        self.cache = String::from("");
                        self.line = 0;
                        self.column = 0;
                        break;
                    }
                }
            }
            State::Punctuator => {
                while let Some(char) = self.code.peek() {
                    if token::is_punctuator(&char) {
                        self.cache.push_str(&char);
                        let char_o = self.code.next();
                        if let Some(char_r) = char_o {
                            if self.line == 0 {
                                self.line = char_r.line;
                                self.column = char_r.column;
                            }
                        }
                    } else {
                        self.state = State::Start;
                        let t_type = token::get_punctuator_type(&self.cache);
                        self.accept(Token {
                            value: self.cache.clone(),
                            category: t_type,
                            line: self.line,
                            column: self.column
                        });
                        self.cache = String::from("");
                        self.line = 0;
                        self.column = 0;
                        break;
                    }
                }
            }
            _ => {
                // self.start();
            }
        }
        if self.state == State::Start {
            self.start();
        }
        
    }
    pub fn start (&mut self) {

        // println!("{:?}", self.code);
        if let Some(start_char) = self.code.peek() {
            let chars: Vec<char> = start_char.chars().collect();
            if chars.len() > 1 {
                self.state = State::String;
            } else {
                if token::is_punctuator(&start_char) {
                    self.state = State::Punctuator;
                } else if token::is_alphabetic(chars[0]) {
                    self.state = State::String;
                } else if token::is_numeric(&start_char) {
                    self.state = State::Number;
                } else {
                    // other chars, unhandled now
                    self.code.next();
                }
            }
            self.forwrad()
        } else {
            println!("lexer ending");
            println!("{:#?}", self.list);
        }
        
    }
    pub fn accept (&mut self, tk: Token) {
        self.list.push(tk);
    }
}