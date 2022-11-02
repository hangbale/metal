use super::variable::Identifier;
#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Bin(BinaryExp),
    Unary(UnaryExpr),
    Identifier(Identifier)
}

#[derive(Debug)]
pub enum Literal {
    String(Str),
    Number(Number),
    Null,
    Undefined,
    Boolean(Boolean)
}
#[derive(Debug)]
struct Str {
    value: String
}
#[derive(Debug)]
struct Number {
    value: f64
}
struct Null {
}
#[derive(Debug)]
struct Boolean {
    value: bool
}
impl From<String> for Literal {
    #[inline]
    fn from(st: String) -> Self {
        Self::String(Str {
            value: st
        })
    }
}
impl From<f64> for Literal {
    #[inline]
    fn from(n: f64) -> Self {
        Self::Number(Number { value: n })
    }
}
impl From<bool> for Literal {
    #[inline]
    fn from(n: bool) -> Self {
        Self::Boolean(Boolean { value: n })
    }
}

impl From<Literal> for Expression {
    #[inline]
    fn from(n: Literal) -> Expression {
        Self::Literal(n)
    }
}


#[derive(Debug)]
pub enum BinaryOpt {
    Add,
    Sub,
    Div,
    Mul,
    Lt,
    Gt
}

#[derive(Debug)]
pub enum UnaryOp {
    Add,
    Sub,
    Bang
}

#[derive(Debug)]
pub struct BinaryExp {
    pub operator: BinaryOpt,
    pub left: Box<Expression>,
    pub right: Box<Expression>
}
#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: UnaryOp,
    pub argument: Box<Expression>
}