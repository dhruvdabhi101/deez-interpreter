use crate::lexer;
use crate::lexer::lexer::{Lexer};


pub fn main() {
    println!("This is the Monkey Language");
    println!("Feel free to type in commands");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut l = Lexer::new(input);
        loop {
            let tok = l.next_token().unwrap();
            if tok == lexer::lexer::Token::Eof {
                break;
            }
            println!("{:?}", tok);
        }
    }

}
