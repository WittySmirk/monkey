use crate::lexer;
use crate::token;

#[test]
fn test_next_token() {
    let input = "let five = 5;
    let ten = 10;

    let add = fn(x,y) {
        x+y;
    };
    let result = add(five,ten);
    ";

    let checks: Vec<token::Token> = vec![
        token::Token {
            t_type: token::TokenType::LET,
            literal: String::from("let"),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("five"),
        },
        token::Token {
            t_type: token::TokenType::ASSIGN,
            literal: String::from("="),
        },
        token::Token {
            t_type: token::TokenType::INT,
            literal: String::from("5"),
        },
        token::Token {
            t_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        token::Token {
            t_type: token::TokenType::LET,
            literal: String::from("let"),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("ten"),
        },
        token::Token {
            t_type: token::TokenType::ASSIGN,
            literal: String::from("="),
        },
        token::Token {
            t_type: token::TokenType::INT,
            literal: String::from("10"),
        },
        token::Token {
            t_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        token::Token {
            t_type: token::TokenType::LET,
            literal: String::from("let"),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("add"),
        },
        token::Token {
            t_type: token::TokenType::ASSIGN,
            literal: String::from("="),
        },
        token::Token {
            t_type: token::TokenType::FUNCTION,
            literal: String::from("fn"),
        },
        token::Token {
            t_type: token::TokenType::LPARAN,
            literal: String::from("("),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("x"),
        },
        token::Token {
            t_type: token::TokenType::COMMA,
            literal: String::from(","),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("y"),
        },
        token::Token {
            t_type: token::TokenType::RPARAN,
            literal: String::from(")"),
        },
        token::Token {
            t_type: token::TokenType::LBRACE,
            literal: String::from("{"),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("x"),
        },
        token::Token {
            t_type: token::TokenType::PLUS,
            literal: String::from("+"),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("y"),
        },
        token::Token {
            t_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        token::Token {
            t_type: token::TokenType::RBRACE,
            literal: String::from("}"),
        },
        token::Token {
            t_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        token::Token {
            t_type: token::TokenType::LET,
            literal: String::from("let"),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("result"),
        },
        token::Token {
            t_type: token::TokenType::ASSIGN,
            literal: String::from("="),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("add"),
        },
        token::Token {
            t_type: token::TokenType::LPARAN,
            literal: String::from("("),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("five"),
        },
        token::Token {
            t_type: token::TokenType::COMMA,
            literal: String::from(","),
        },
        token::Token {
            t_type: token::TokenType::IDENT,
            literal: String::from("ten"),
        },
        token::Token {
            t_type: token::TokenType::LPARAN,
            literal: String::from(")"),
        },
        token::Token {
            t_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
    ];

    let mut l = lexer::Lexer::new(input.to_string());

    for t in checks {
        let curr = l.next_token();

        // Todo: document this better
        assert_eq!(curr.t_type, t.t_type);
        assert_eq!(curr.literal, t.literal);
    }
}
