use crate::lexer::lexer::*;

#[allow(dead_code)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub enum Statement {
    LetStatement {
        token: Token, // Let  Token 
        name: Identifier, 
    }
}
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub struct Identifier {
    pub token: Token, // Ident Token
    pub value: String,
}


#[allow(dead_code)]
pub enum Expression {
}
