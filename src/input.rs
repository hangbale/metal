use std::str::Chars;
use std::iter::Peekable;
#[derive(Debug)]
pub struct Char {
    pub text: String,
    pub line: u64,
    pub column: u64
}

#[derive(Debug)]
pub struct Code<'a> {
    sources: Chars<'a>,
    line_cursor: u64,
    iter: Peekable<Chars<'a>>,
    column_cursor: u64,
}

impl<'a> Code<'a> {
    pub fn new (code: &'a str) -> Self {
        Self {
            sources: code.chars(),
            iter: code.chars().peekable(),
            line_cursor: 1,
            column_cursor: 1,
        }
    }
    pub fn next (&mut self) -> Option<char> {
        let ch = self.iter.next();
        match ch {
            Some(_) => self.column_cursor = self.column_cursor + 1,
            None => {}
        }
        ch
 
    }
    pub fn peek (&mut self) -> Option<char> {
        let pk = self.iter.peek();
        if let Some(ch) = pk {
            Some(*ch)
        } else {
            None
        }
    } 
    pub fn get_position (&self) -> (u64, u64) {
        (self.line_cursor, self.column_cursor)
    }
}
