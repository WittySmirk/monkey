use crate::lexer;
use crate::token;
use std::io;
use std::io::Write;

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().expect("error: failed to flush");
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: enter valid monkey code");
        let mut lex: lexer::Lexer = lexer::Lexer::new(input);
        loop {
            let tok: token::Token = lex.next_token();
            if tok.t_type == token::TokenType::EOF {
                break;
            }
            println!("{}", tok);
        }
    }
}
