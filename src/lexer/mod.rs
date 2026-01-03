use crate::token;

pub struct Lexer {
    input: &'static str,
}

impl Lexer {
    pub fn new(input: &'static str) -> Self {
        Self { input }
    }
    pub fn next_token(&self) -> token::TokenType {
        token::TokenType::EOF
    }
}
