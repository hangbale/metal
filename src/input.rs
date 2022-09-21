use std::env;
use std::io::{ BufReader, Lines};
use std::io::prelude::*;
use std::fs::File;
use unicode_segmentation::{UnicodeSegmentation};

#[derive(Debug)]
pub struct Char {
    pub text: String,
    pub line: u64,
    pub column: u64
}

#[derive(Debug)]
pub struct Code {
    input: Lines<BufReader<File>>,
    current_line: Option<Vec<String>>, // None: end of file, []: should look next
    line_cursor: u64,
    column_cursor: u64
}

impl Code {
    pub fn new () -> Self {
        let arg = env::args().nth(1).expect("require a js file");
        let file = File::open(arg).expect("failed to open file");
        let buff = BufReader::new(file).lines();
        Self {
            input: buff,
            current_line: Some(vec![]),
            line_cursor: 0,
            column_cursor: 1
        }
    }
    pub fn next (&mut self) -> Option<Char> {
        if let None = self.current_line {
            return None
        } else if self.current_line == Some(vec![]) {
            println!("empty line");
            self.next_line();
        }
        self.next_char()
    }
    pub fn peek (&mut self) -> Option<String> {
        if let Some(v) = &mut self.current_line {
            if let Some(st) = v.last() {
                Some(st.clone())
            } else {
                self.next_line();
                return self.peek();
            }
        } else {
            None
        }
    } 
    fn next_char (&mut self) -> Option<Char> {
        if let Some(line_str) = &mut self.current_line {
            let char_opt = line_str.pop();
            if let Some(char) = char_opt {
                // println!("[{}][{}]: {}", self.line_cursor, self.column_cursor, char);
                let ret = Char { 
                    text: char,
                    line: self.line_cursor,
                    column: self.column_cursor
                };
                self.column_cursor = self.column_cursor + 1;
                Some(ret)
            } else {
                // None
                println!("end of line");
                self.next_line();
                let next = self.next_char();
                return next;
            }
        } else {
            None
        }
   
    }
    pub fn next_line (&mut self) {
        let lines = &mut self.input;
        let next_line = lines.next();
        
        if let Some(Ok(line_strs)) = next_line {
            self.line_cursor = self.line_cursor + 1;
            self.column_cursor = 1;
            let mut ret = line_strs.graphemes(true).map(|x| x.to_string())
            .collect::<Vec<String>>();
            ret.reverse();
            self.current_line = Some(ret);
        } else {
            println!("end of file");
            self.current_line = None;
        }
    }
    pub fn print (self) {
        let mut line_number: usize = 1;
        let mut column_number: u32 = 1;
        let input = self.input;
        for l in input {
            let line = l.unwrap();
            let mut chs = line.graphemes(true);
            while let Some(ch) = chs.next()  {
                println!("{}: {}", column_number, ch);
                column_number = column_number + 1;
            }
            line_number = line_number + 1;
        }
    }
}
