use crate::token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        // Default values
        let mut l = Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        return l;
    }
    pub fn next_token(&mut self) -> token::Token {
        let mut tok: token::Token = token::Token {
            t_type: token::TokenType::ILLEGAL,
            literal: self.ch.to_string(),
        };

        self.skip_whitespace();

        match self.ch {
            '=' => {
                tok = token::Token {
                    t_type: token::TokenType::ASSIGN,
                    literal: self.ch.to_string(),
                }
            }
            ';' => {
                tok = token::Token {
                    t_type: token::TokenType::SEMICOLON,
                    literal: self.ch.to_string(),
                }
            }
            '(' => {
                tok = token::Token {
                    t_type: token::TokenType::LPARAN,
                    literal: self.ch.to_string(),
                }
            }
            ')' => {
                tok = token::Token {
                    t_type: token::TokenType::RPARAN,
                    literal: self.ch.to_string(),
                }
            }
            '{' => {
                tok = token::Token {
                    t_type: token::TokenType::LBRACE,
                    literal: self.ch.to_string(),
                }
            }
            '}' => {
                tok = token::Token {
                    t_type: token::TokenType::RBRACE,
                    literal: self.ch.to_string(),
                }
            }
            '+' => {
                tok = token::Token {
                    t_type: token::TokenType::PLUS,
                    literal: self.ch.to_string(),
                }
            }
            ',' => {
                tok = token::Token {
                    t_type: token::TokenType::COMMA,
                    literal: self.ch.to_string(),
                }
            }
            '\0' => {
                tok = token::Token {
                    t_type: token::TokenType::EOF,
                    literal: self.ch.to_string(),
                }
            }
            _ => {
                if Lexer::is_letter(self.ch) {
                    let ident: String = self.read_identifier();
                    tok.literal = ident.clone(); // TODO: Maybe fix this performance hit
                    tok.t_type = token::lookup_ident(ident);
                    return tok;
                } else if Lexer::is_digit(self.ch) {
                    tok.t_type = token::TokenType::INT;
                    tok.literal = self.read_number();
                    return tok;
                }
            }
        }
        self.read_char();
        return tok;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;

        while Lexer::is_letter(self.ch) {
            self.read_char();
        }
        return self.input[pos..self.position].to_string();
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char();
        }
        return self.input[pos..self.position].to_string();
    }

    fn is_letter(c: char) -> bool {
        return 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_';
    }

    fn is_digit(c: char) -> bool {
        return '0' <= c && c <= '9';
    }
}
