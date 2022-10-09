use crate::lexer::token::{TokenType};
// in javascript, number is a double type value
// an IEEE 754 64-bit double has 52 bits of mantissa,
// so the largest safe decimal number is 2^53(excluded)
// octal: 377777777777777777
// hex: 1fffffffffffff
// binary: 11111111111111111111111111111111111111111111111111111
// for the purpose of simplification, just store arbitrarily number in f64

pub fn parse_binary_number(s: &str) -> Result<f64, String> {
    let bs = s.as_bytes();
    let mut result: f64 = 0.0;
    for c in bs {
        match c {
            b'0'..=b'1' => {
                let ct = c - b'0';
                result = result * 2.0 + ct as f64;
            }
            _ => {
                return Err("invalid number".to_string());
            }
        }
    }
    Ok(result)
}
pub fn parse_decimal_int(s: &str) -> Result<f64, String> {
    let bs = s.as_bytes();
    let mut result: f64 = 0.0;
    for c in bs {
        match c {
            b'0'..=b'9' => {
                let ct = c - b'0';
                result = result * 10.0 + ct as f64;
            }
            _ => {
                return Err("invalid number".to_string());
            }
        }
    }
    Ok(result)
}
pub fn parse_octal_number(s: &str) -> Result<f64, String> {
    let bs = s.as_bytes();
    let mut result: f64 = 0.0;
    for c in bs {
        match c {
            b'0'..=b'7' => {
                let ct = c - b'0';
                result = result * 8.0 + ct as f64;
            }
            _ => {
                return Err("invalid number".to_string());
            }
        }
    }
    Ok(result)
}
pub fn parse_hex_number(s: &str) -> Result<f64, String> {
    let bs = s.as_bytes();
    let mut result: f64 = 0.0;
    for c in bs {
        match c {
            b'0'..=b'9' => {
                let ct = c - b'0';
                result = result * 16.0 + ct as f64;
            }
            b'a'..=b'f' => {
                let ct = c - b'a' + 10;
                result = result * 16.0 + ct as f64;
            }
            b'A'..=b'F' => {
                let ct = c - b'A' + 10;
                result = result * 16.0 + ct as f64;
            }
            _ => {
                return Err("invalid number".to_string());
            }
        }
    }
    Ok(result)
}

pub fn parse_numeric(s: &str, tp: &TokenType) -> Result<f64, String> {
    match tp {
        TokenType::NUMERIC_LITERAL_BINARY => parse_binary_number(s),
        TokenType::NUMERIC_LITERAL_DECIMAL => parse_decimal_int(s),
        TokenType::NUMERIC_LITERAL_OCTAL => parse_octal_number(s),
        TokenType::NUMERIC_LITERAL_HEX => parse_hex_number(s),
        _ => Err("invalid number".to_string())
    }
}