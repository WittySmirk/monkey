#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
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
            TokenType::ILLEGAL => te = "ILLEGAL",
            TokenType::EOF => te = "EOF",

            //Identifiers and Ints
            TokenType::IDENT => te = "IDENT",
            TokenType::INT => te = "INT",

            //Operators
            TokenType::ASSIGN => te = "ASSIGN",
            TokenType::PLUS => te = "PLUS",
            TokenType::MINUS => te = "MINUS",
            TokenType::BANG => te = "BANG",
            TokenType::ASTERISK => te = "ASTERISK",
            TokenType::SLASH => te = "SLASH",
            TokenType::LT => te = "LT",
            TokenType::GT => te = "GT",
            TokenType::EQ => te = "EQ",
            TokenType::NE => te = "NE",
            //Delimeters
            TokenType::COMMA => te = "COMMA",
            TokenType::SEMICOLON => te = "SEIMCOLON",

            TokenType::LPARAN => te = "LPARAN",
            TokenType::RPARAN => te = "RPARAN",
            TokenType::LBRACE => te = "LBRACE",
            TokenType::RBRACE => te = "RBRACE",

            //Keywords
            TokenType::FUNCTION => te = "FUNCTION",
            TokenType::LET => te = "LET",
            TokenType::TRUE => te = "TRUE",
            TokenType::FALSE => te = "FALSE",
            TokenType::IF => te = "IF",
            TokenType::ELSE => te = "ELSE",
            TokenType::RETURN => te = "RETURN",
        }

        write!(f, "{}", te)
    }
}

// String or str for this?
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

impl Clone for Token {
    fn clone(&self) -> Self {
        Self {
            t_type: self.t_type,
            literal: self.literal.clone(),
        }
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
