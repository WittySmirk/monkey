use std::collections::HashMap;

use crate::ast;
use crate::lexer;
use crate::token;

type prefix_parse_fn<T> = fn(T) -> ast::Expression;
type infix_parse_fn<T> = fn(T, left: ast::Expression) -> ast::Expression;

enum Precedince {
    BLANK = 0,
    LOWEST,
    EQUALS,      // =
    LESSGREATER, // < or >
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // x()
}

pub struct Parser {
    lexer: lexer::Lexer,
    curr_token: token::Token,
    peek_token: token::Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: lexer::Lexer) -> Self {
        let c: token::Token = lexer.next_token();
        let p: token::Token = lexer.next_token();

        Self {
            lexer,
            curr_token: c,
            peek_token: p,
            errors: Vec::new(),
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
            token::TokenType::RETURN => {
                return Some(ast::Statement::Return(self.parse_return_statement()?))
            }
            _ => {
                return Some(ast::Statement::Expression(
                    self.parse_expression_statement()?,
                ))
            }
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
            value: None,
        };

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
            self.peek_error(t);
            return false;
        }

        self.next_token();
        true
    }

    fn peek_error(&mut self, t_type: token::TokenType) {
        let err = format!(
            "parser error: expected peek type to be {} but was {}",
            t_type, self.peek_token.t_type
        );

        self.errors.push(err);
    }
    fn parse_return_statement(&mut self) -> Option<ast::ReturnStatement> {
        let statement: ast::ReturnStatement = ast::ReturnStatement {
            token: self.curr_token.clone(),
            return_value: None,
        };

        while self.curr_token.t_type != token::TokenType::SEMICOLON {
            self.next_token();
        }
        Some(statement)
    }

    fn parse_expression_statement(&mut self) -> Option<ast::ExpressionStatement> {
        let mut statement: ast::ExpressionStatement = ast::ExpressionStatement {
            token: self.curr_token.clone(),
            expression: None,
        };

        statement.expression = self.parse_expression(Precedince::LOWEST);

        if self.peek_token.t_type == token::TokenType::SEMICOLON {
            self.next_token();
        }

        Some(statement)
    }

    fn parse_expression(&mut self, precidence: Precedince) -> Option<ast::Expression> {
        match self.curr_token.t_type {
            token::TokenType::IDENT => return Some(self.parse_identifier()),
            _ => return None,
        }
    }

    fn parse_identifier(&mut self) -> ast::Expression {
        ast::Expression::Identifier(ast::Identifier {
            token: self.curr_token.clone(),
            value: self.curr_token.literal.clone(),
        })
    }
}
