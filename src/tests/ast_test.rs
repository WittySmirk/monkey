use crate::ast;
use crate::ast::Node;
use crate::token;

#[test]
fn test_string() {
    let program: ast::Program = ast::Program {
        statements: vec![ast::Statement::Let(ast::LetStatement {
            token: token::Token {
                t_type: token::TokenType::LET,
                literal: String::from("let"),
            },
            name: ast::Identifier {
                token: token::Token {
                    t_type: token::TokenType::IDENT,
                    literal: String::from("myVar"),
                },
                value: String::from("myVar"),
            },
            value: Some(ast::Expression::Identifier(ast::Identifier {
                token: token::Token {
                    t_type: token::TokenType::IDENT,
                    literal: String::from("anotherVar"),
                },
                value: String::from("anotherVar"),
            })),
        })],
    };
    if program.string() != "let myVar = anotherVar;" {
        assert!(false, "program.string() wrong, got {}", program.string());
    }
}
