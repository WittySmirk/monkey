use std::mem;

use crate::ast;
use crate::lexer;
use crate::token;

pub struct Parser {
    lexer: lexer::Lexer,
    curr_token: token::Token,
    peek_token: token::Token,
}

impl Parser {
    pub fn new(mut lexer: lexer::Lexer) -> Self {
        let c: token::Token = lexer.next_token();
        let p: token::Token = lexer.next_token();
        Self {
            lexer,
            curr_token: c,
            peek_token: p,
        }
    }

    pub fn next_token(&mut self) {
        self.curr_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token());
    }

    pub fn parse_program(&mut self) -> Option<ast::Program> {
        let mut program: ast::Program = ast::Program {
            statements: Vec::<ast::Statement>::new(),
        };

        while self.curr_token.t_type != token::TokenType::EOF {
            match self.parse_statement() {
                Some(s) => {
                    program.statements.push(s);
                }
                None => {}
            };
            self.next_token();
        }

        if program.statements.is_empty() {
            return None;
        }
        Some(program)
    }

    fn parse_statement(&mut self) -> Option<ast::Statement> {
        match self.curr_token.t_type {
            token::TokenType::LET => return Some(ast::Statement::Let(self.parse_let_statement()?)),
            _ => return None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<ast::LetStatement> {
        // TODO: Lots of cloning here, could maybe tidy up
        let mut statement: ast::LetStatement = ast::LetStatement {
            token: self.curr_token.clone(),
            name: ast::Identifier {
                token: token::Token {
                    t_type: token::TokenType::IDENT,
                    literal: String::from("\0"),
                },
                value: String::from("\0"),
            },
            value: ast::Expression::Identifier(ast::Identifier {
                token: token::Token {
                    t_type: token::TokenType::IDENT,
                    literal: String::from("\0"),
                },
                value: String::from("\0"),
            }),
        };
        statement.token = self.curr_token.clone();

        if !self.expect_peek(token::TokenType::IDENT) {
            return None;
        }

        statement.name = ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.as_str().to_string(),
        };
        if !self.expect_peek(token::TokenType::ASSIGN) {
            return None;
        }

        while self.curr_token.t_type != token::TokenType::SEMICOLON {
            self.next_token();
        }

        Some(statement)
    }

    fn expect_peek(&mut self, t: token::TokenType) -> bool {
        if self.peek_token.t_type != t {
            return false;
        }

        self.next_token();
        true
    }
}
