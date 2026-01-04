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
}

pub struct Token {
    pub t_type: TokenType,
    pub literal: String,
}
const known: [(&str, TokenType); 2] = [("fn", TokenType::FUNCTION), ("let", TokenType::LET)];

pub fn lookup_ident(ident: String) -> TokenType {
    for k in known {
        if k.0 == ident {
            return k.1;
        }
    }
    return TokenType::IDENT;
}
