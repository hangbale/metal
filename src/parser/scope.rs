use super::error::ParseError;

pub enum ScopeType {
    ScopeTop = 1,
    ScopeFunction = 2,
}

pub struct Scope {
    var_list: Vec<String>,
    type_flag: u8
}

impl Scope {
    pub fn new() -> Self {
        Self {
            var_list: vec![],
            type_flag: 1
        }
    }
    pub fn declare(&mut self, name: &str) -> Result<(), ParseError> {
        if self.var_list.iter().any(|x| x == name) {
            Err(ParseError::AlreadyDeclared(String::from(name)))
        } else {
            self.var_list.push(name.to_string());
            Ok(())
        }
    }
}