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

impl TokenType {
    pub fn as_str(self) -> &'static str {
        match self {
            TokenType::ILLEGAL => "ILLEGAL",
            TokenType::EOF => "EOF",
            TokenType::IDENT => "IDENT",
            TokenType::INT => "INT",
            TokenType::ASSIGN => "=",
            TokenType::PLUS => "+",
            TokenType::COMMA => ",",
            TokenType::SEMICOLON => ";",
            TokenType::LPARAN => "(",
            TokenType::RPARAN => ")",
            TokenType::LBRACE => "{",
            TokenType::RBRACE => "}",
            TokenType::FUNCTION => "FUNCTION",
            TokenType::LET => "LET",
        }
    }
}
