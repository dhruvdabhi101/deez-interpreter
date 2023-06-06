use crate::parser::ast::*;
use crate::lexer::lexer::*;


pub struct Parser{
    pub lex: Lexer,
    pub cur_token: Token,
    pub peek_token: Token,
    pub erros: Vec<String>
}

impl Parser { 
    pub fn new(lex: Lexer) -> Parser {
        let mut p = Parser {
            lex: lex,
            cur_token: Token::Eof,
            peek_token: Token::Eof,
            erros: Vec::new()
        };
        p.next_token();
        p.next_token();
        return p;
    }

    pub fn Errors(&self) -> Vec<String> {
        self.erros.clone()
    }

    pub fn peek_error(&mut self, token: Token) {
        let msg = format!("expected next token to be {:?}, got {:?} instead", token, self.peek_token);
        self.erros.push(msg);
    }


    pub fn next_token(&mut self){
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token().unwrap_or(Token::Eof);
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut program: Vec<Statement> = Vec::new();
        while self.cur_token != Token::Eof {
            let stmt = self.parse_statement();
            if let Some(stmt) = stmt {
                program.push(stmt);
            }
            self.next_token();
        }
        return program;
    }
    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token {
            Token::Let =>{
                return Some(self.parse_let_statement().unwrap());
            },
            _ => None
        }
    }



    fn parse_let_statement(&mut self) -> Option<Statement> {
        let current_token = self.cur_token.clone();


        if !self.expect_peek(Token::Ident("".into())) {
            return None;

        }

        let stmt:Statement  = Statement::LetStatement { token: current_token, 
            name: Identifier { token: self.cur_token.clone(), 
                value: match &self.cur_token {
                    Token::Ident(s) => s.to_string(),
                    _ => "".into()
                }
                    
            } 
        };
            


        if !self.expect_peek(Token::Assign) {
            return None;
        }


        while !self.cur_token_is(Token::Semicolon) {
            self.next_token();
        }


        return Some(stmt);
    }


    fn expect_peek(&mut self, token:Token) -> bool {
        // check if varient is same or not 
        match self.peek_token {
            Token::Ident(_) => {
                if let Token::Ident(_) = token {
                    self.next_token();
                    return true;
                } else {
                    self.peek_error(token);
                    return false;
                }
            },
            _ => {
                if self.peek_token == token {
                    self.next_token();
                    return true;
                } else {
                    self.peek_error(token);
                    return false;
                }
            }
        }

    }

    fn cur_token_is(&self, token:Token) -> bool {
        if self.cur_token == token {
            true 
        } else {
            false
        }
    }


}



// test 
#[cfg(test)]

mod test {
    use super::*;
    use anyhow::Result;
    use crate::lexer::lexer::*;
    use crate::parser::ast::*;
    use crate::parser::parser::*;
    type Err = anyhow::Error;
    #[test]
    fn test_let_statements() -> Result<()> {
        let input = r#"
        let x = 5;
        let y = 10;
        let foobar = 838383;
        "#;
        let lex = Lexer::new(input.into());
        let mut p = Parser::new(lex);
        let program = p.parse();

        if program.len() == 0 {
            panic!("parse return 0");
        }

        if program.len() != 3 {
            panic!("program does not contain 3 statements, got={}", program.len());
        }


        check_parser_errors(&p);

        assert_eq!(program.len(), 3);
        let tests = vec![
            "x",
            "y",
            "foobar"
        ];
        for (i, test) in tests.iter().enumerate() {
            let stmt = &program[i];
            test_let_statement(stmt, test)?;
        }

        Ok(())
    }

    fn test_let_statement(stmt: &Statement, name: &str) -> Result<()> {

        match stmt {
            Statement::LetStatement { token: _, name: ident} => {
                println!("ident: {:?}", ident.value);
                assert_eq!(ident.value.as_str(), name);
            },
            _ => {
                return anyhow::bail!("not let statement");
            }
        }
       Ok(()) 

    }
    fn check_parser_errors(p: &Parser) {
       let error = p.Errors();
        if error.len() == 0 {
            return 
        }
        println!("parser has {} errors", error.len());
        for msg in error {
            println!("parser error: {}", msg);
        }
        panic!();
    }

}

