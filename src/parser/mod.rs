mod scope;
mod error;
use crate::{lexer::{lexer::Lexer, token::{Token, TokenType, TokenMatcher}}, ast::expression::{Expression, Literal}};
use crate::ast::{variable::{VariableDeclaration, Variable}};
pub struct Parser<'a> {
    scope_stack: Vec<scope::Scope>,
    lexer: Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { scope_stack: vec![], lexer: lexer }
    }
    fn next(&mut self) -> Result<Token, error::ParseError> {
        self.lexer.advance().map_err(|e| error::ParseError::LexerError(e))
    }
    fn next_check(&mut self, tp: TokenMatcher) -> Result<Token, error::ParseError> {
        let n = self.next()?;
        match tp {
            TokenMatcher::Single(stp) => {
                if n.category == n.category {
                    return Ok(n);
                }
                Err(error::ParseError::UnexpectedToken(n.value))
            }
            TokenMatcher::List(stp) => {
                if stp.iter().any(|x| *x == n.category) {
                    return Ok(n);
                }
                Err(error::ParseError::UnexpectedToken(n.value))
            }
        }
     
    }
    pub fn parse(&mut self) -> Result<(), error::ParseError> {
        let nt = self.lexer.advance();
        match nt {
            Ok(tt) => {
                match tt.category {
                    TokenType::LET => {
                        let t = self.parse_decl();
                        println!("{:#?}", t);
                    }
                    _ => ()
                }
            }
            Err(e) => {}
        }
    
        Ok(())
    }
    fn parse_decl(&mut self) -> Result<VariableDeclaration, error::ParseError> {
        let ident = self.next_check(TokenMatcher::from(TokenType::Identifier))?;
        
        self.next_check(TokenMatcher::from(TokenType::ASSIGN))?;
        let exp = self.parse_literal()?;
        let mut decls = vec![];
        decls.push(Variable::new(ident.value, Some(exp)));
        Ok(VariableDeclaration {
            list: decls
        })
    }
    fn parse_literal(&mut self) -> Result<Expression, error::ParseError>{
        let n = self.next()?;
        match n.category {
            TokenType::STRING_LITERAL => {
                let lt = Literal::from(n.value);
                return Ok(Expression::from(lt));
            }
            TokenType::NULL_LITERAL => {
                return Ok(Expression::from(Literal::Null));
            }
            TokenType::NUMERIC_LITERAL => {
                return Ok(Expression::from(Literal::from(n.value)));
            }
            TokenType::TRUE_LITERAL | TokenType::FALSE_LITERAL => {
                return Ok(Expression::from(Literal::from(n.value == "true")));
            }
            _ => {
                return Err(error::ParseError::UnexpectedToken(n.value));
            }
        }
    }
    fn parse_unary(&mut self) -> Result<Expression, error::ParseError> {
        let nt = self.next_check(TokenMatcher::from(vec![
            TokenType::ADD,
            TokenType::SUB,
            TokenType::Bang
        ]))?;
    }
}