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

impl std::fmt::Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut te: &str = "";
        match self {
            TokenType::ILLEGAL => te = "illegal",
            TokenType::EOF => te = "eof",

            //Identifiers and Ints
            TokenType::IDENT => te = "ident",
            TokenType::INT => te = "ident",

            //Operators
            TokenType::ASSIGN => te = "assign",
            TokenType::PLUS => te = "plus",
            TokenType::MINUS => te = "minus",
            TokenType::BANG => te = "bang",
            TokenType::ASTERISK => te = "asterisk",
            TokenType::SLASH => te = "slash",
            TokenType::LT => te = "lt",
            TokenType::GT => te = "gt",
            TokenType::EQ => te = "eq",
            TokenType::NE => te = "ne",
            //Delimeters
            TokenType::COMMA => te = "comma",
            TokenType::SEMICOLON => te = "semicolon",

            TokenType::LPARAN => te = "lparan",
            TokenType::RPARAN => te = "rparan",
            TokenType::LBRACE => te = "lbrace",
            TokenType::RBRACE => te = "rbrace",

            //Keywords
            TokenType::FUNCTION => te = "function",
            TokenType::LET => te = "let",
            TokenType::TRUE => te = "true",
            TokenType::FALSE => te = "false",
            TokenType::IF => te = "if",
            TokenType::ELSE => te = "else",
            TokenType::RETURN => te = "return",
        }

        write!(f, "{}", te)
    }
}

pub struct Token {
    pub t_type: TokenType,
    pub literal: String,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token Type: {} String Literal: {}",
            self.t_type, self.literal
        )
    }
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
