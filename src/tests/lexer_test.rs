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

    !-/*5;
    5 < 10 > 5;

    if (5<10) {
        return true;
    } else {
        return false;
    }

    10 == 10;
    10 != 9;
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
            t_type: token::TokenType::RPARAN,
            literal: String::from(")"),
        },
        token::Token {
            t_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
        token::Token {
            t_type: token::TokenType::BANG,
            literal: String::from("!"),
        },
        token::Token {
            t_type: token::TokenType::MINUS,
            literal: String::from("-"),
        },
        token::Token {
            t_type: token::TokenType::SLASH,
            literal: String::from("/"),
        },
        token::Token {
            t_type: token::TokenType::ASTERISK,
            literal: String::from("*"),
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
            t_type: token::TokenType::INT,
            literal: String::from("5"),
        },
        token::Token {
            t_type: token::TokenType::LT,
            literal: String::from("<"),
        },
        token::Token {
            t_type: token::TokenType::INT,
            literal: String::from("10"),
        },
        token::Token {
            t_type: token::TokenType::GT,
            literal: String::from(">"),
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
            t_type: token::TokenType::IF,
            literal: String::from("if"),
        },
        token::Token {
            t_type: token::TokenType::LPARAN,
            literal: String::from("("),
        },
        token::Token {
            t_type: token::TokenType::INT,
            literal: String::from("5"),
        },
        token::Token {
            t_type: token::TokenType::LT,
            literal: String::from("<"),
        },
        token::Token {
            t_type: token::TokenType::INT,
            literal: String::from("10"),
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
            t_type: token::TokenType::RETURN,
            literal: String::from("return"),
        },
        token::Token {
            t_type: token::TokenType::TRUE,
            literal: String::from("true"),
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
            t_type: token::TokenType::ELSE,
            literal: String::from("else"),
        },
        token::Token {
            t_type: token::TokenType::LBRACE,
            literal: String::from("{"),
        },
        token::Token {
            t_type: token::TokenType::RETURN,
            literal: String::from("return"),
        },
        token::Token {
            t_type: token::TokenType::FALSE,
            literal: String::from("false"),
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
            t_type: token::TokenType::INT,
            literal: String::from("10"),
        },
        token::Token {
            t_type: token::TokenType::EQ,
            literal: String::from("=="),
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
            t_type: token::TokenType::INT,
            literal: String::from("10"),
        },
        token::Token {
            t_type: token::TokenType::NE,
            literal: String::from("!="),
        },
        token::Token {
            t_type: token::TokenType::INT,
            literal: String::from("9"),
        },
        token::Token {
            t_type: token::TokenType::SEMICOLON,
            literal: String::from(";"),
        },
    ];

    let mut l = lexer::Lexer::new(input.to_string());

    for t in checks {
        let curr = l.next_token();

        // TODO: document this better
        assert_eq!(curr.t_type, t.t_type);
        assert_eq!(curr.literal, t.literal);
    }
}
