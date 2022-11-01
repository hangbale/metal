use super::expression::Expression;
#[derive(Debug)]
pub struct Identifier {
    value: String
}
#[derive(Debug)]
pub struct Variable {
    name: Identifier,
    init: Option<Expression>
}
#[derive(Debug)]
pub struct VariableDeclaration {
    pub list: Vec<Variable>
}

impl Variable {
    pub fn new(name: String, exp: Option<Expression>) -> Self {
        Self {
            name: Identifier { value: name },
            init: exp
        }
    } 
}