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
        let mut tok: token::Token;
        /*
                let mut tok: token::Token = token::Token {
                    t_type: token::TokenType::ILLEGAL,
                    literal: "\0".to_string(),
                };
        */
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
                    tok = token::Token {
                        t_type: token::TokenType::IDENT,
                        literal: self.read_identifier(),
                    };
                    return tok;
                } else {
                    tok = token::Token {
                        t_type: token::TokenType::ILLEGAL,
                        literal: self.ch.to_string(),
                    };
                }
            }
        }
        self.read_char();
        return tok;
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

    fn is_letter(c: char) -> bool {
        return 'a' <= c && c <= 'z' || 'A' <= c && c <= 'Z' || c == '_';
    }
}
