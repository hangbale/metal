use std::str::Chars;
use std::iter::Peekable;

// line ending
pub const LF: char = '\u{a}';
pub const CR: char = '\u{d}';

fn is_line_break (s: char) -> bool {
    return s == LF || s == CR;
}
#[derive(Debug)]
pub struct Char {
    pub text: String,
    pub line: u64,
    pub column: u64
}

#[derive(Debug)]
pub struct Code<'a> {
    pub line_cursor: u64,
    iter: Peekable<Chars<'a>>,
    pub column_cursor: u64,
    pub column_start: u64
}

impl<'a> Code<'a> {
    pub fn new (code: &'a str) -> Self {
        Self {
            iter: code.chars().peekable(),
            line_cursor: 1,
            column_cursor: 1,
            column_start: 1
        }
    }
    pub fn next (&mut self) -> Option<char> {
        let nt = self.iter.next();
        match nt {
            Some(ch) => {
                if is_line_break(ch) {
                    self.line_cursor = self.line_cursor + 1;
                    self.column_cursor = 1;
                } else {
                    self.column_cursor = self.column_cursor + 1
                }
            },
            None => {}
        }
        nt
    }
    pub fn peek (&mut self) -> Option<char> {
        let pk = self.iter.peek();
        if let Some(ch) = pk {
            Some(*ch)
        } else {
            None
        }
    }
}
