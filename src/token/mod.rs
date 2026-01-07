#[derive(PartialEq, Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,

    //Identifiers and Ints
    IDENT,
    INT,

    //Operators
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NE,

    //Delimeters
    COMMA,
    SEMICOLON,

    LPARAN,
    RPARAN,
    LBRACE,
    RBRACE,

    //Keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}

pub struct Token {
    pub t_type: TokenType,
    pub literal: String,
}

const KNOWN: [(&str, TokenType); 7] = [
    ("fn", TokenType::FUNCTION),
    ("let", TokenType::LET),
    ("true", TokenType::TRUE),
    ("false", TokenType::FALSE),
    ("if", TokenType::IF),
    ("else", TokenType::ELSE),
    ("return", TokenType::RETURN),
];

pub fn lookup_ident(ident: String) -> TokenType {
    for k in KNOWN {
        if k.0 == ident {
            return k.1;
        }
    }
    return TokenType::IDENT;
}
