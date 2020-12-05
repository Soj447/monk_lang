use crate::token::{TokenType, Token};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    current_char: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {input: input, position: 0, read_position: 0, current_char: '\0'};
        lexer.read_char();
        lexer
    }

    fn read_char(&mut self) -> () {
        if self.read_position >= self.input.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    fn peak(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.eat_whitespace();
        let mut literal = self.current_char.to_string();
        let token_type = match self.current_char {
            '=' => {
                if self.peak() == '=' {
                   let ch = self.current_char;
                   self.read_char();
                   literal = format!("{}{}", ch, self.current_char);
                   TokenType::Equality
                } else {
                    TokenType::Assign
                }
            },
            '!' => {
                if self.peak() == '=' {
                    let ch = self.current_char;
                    self.read_char();
                    literal = format!("{}{}", ch, self.current_char);
                    TokenType::NotEqual
                } else {
                    TokenType::Bang
                }
            }
            '>' => TokenType::Gt,
            '<' => TokenType::Lt,
            ';' => TokenType::Semicolon,
            '(' => TokenType::Lparen,
            ')' => TokenType::Rparen,
            ',' => TokenType::Comma,
            '+' => TokenType::Plus,
            '-' => TokenType::Minus,
            '*' => TokenType::Asterisk,
            '/' => TokenType::Slash,
            '{' => TokenType::Lbrace,
            '}' => TokenType::Rbrace,
            '\0' => TokenType::EOF,
            ch if ch.is_alphabetic()  => {
                literal = self.read_identifier().to_string();
                match literal.as_str() {
                    "fn" => TokenType::Function,
                    "let" => TokenType::Let,
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    "if" => TokenType::If,
                    "else" => TokenType::Else,
                    "return" => TokenType::Return,
                    _ => TokenType::Ident
                }
            }
            ch if ch.is_digit(10) => {
                literal = self.read_number().to_string();
                TokenType::Int
            }
            _ => TokenType::Illegal
        };
        if !vec![TokenType::Function, TokenType::Let, TokenType::Ident, TokenType::Int].contains(&token_type) {
            self.read_char();
        };
        Token{t_type: token_type, literal: literal}
    }

    fn read_identifier(&mut self) -> &str {
        let current_position = self.position;
        while self.current_char.is_alphabetic() {
            self.read_char();
        }
        &self.input[current_position..self.position]
    }
    
    fn read_number(&mut self) -> &str {
        let current_position = self.position;
        while self.current_char.is_digit(10) {
            self.read_char();
        };
        &self.input[current_position..self.position]
    }
    
    fn eat_whitespace(&mut self) -> () {
        while vec![' ', '\t', '\r'].contains(&self.current_char) {
            self.read_char()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Case {
        expected_type: TokenType,
        expected_literal: String,
    }
    
    #[test]
    fn test_next_token() {
       let input = "=+(){},;let+dondon+fn test fn notakewword 1 12 == != ! - * / > <";

       let cases = vec![
           Case {expected_type: TokenType::Assign, expected_literal: String::from("=")},
           Case {expected_type: TokenType::Plus, expected_literal: String::from("+")},
           Case {expected_type: TokenType::Lparen, expected_literal: String::from("(")},
           Case {expected_type: TokenType::Rparen, expected_literal: String::from(")")},
           Case {expected_type: TokenType::Lbrace, expected_literal: String::from("{")},
           Case {expected_type: TokenType::Rbrace, expected_literal: String::from("}")},
           Case {expected_type: TokenType::Comma, expected_literal: String::from(",")},
           Case {expected_type: TokenType::Semicolon, expected_literal: String::from(";")},
           Case {expected_type: TokenType::Let, expected_literal: String::from("let")},
           Case {expected_type: TokenType::Plus, expected_literal: String::from("+")},
           Case {expected_type: TokenType::Ident, expected_literal: String::from("dondon")},
           Case {expected_type: TokenType::Plus, expected_literal: String::from("+")},
           Case {expected_type: TokenType::Function, expected_literal: String::from("fn")},
           Case {expected_type: TokenType::Ident, expected_literal: String::from("test")},
           Case {expected_type: TokenType::Function, expected_literal: String::from("fn")},
           Case {expected_type: TokenType::Ident, expected_literal: String::from("notakewword")},
           Case {expected_type: TokenType::Int, expected_literal: String::from("1")},
           Case {expected_type: TokenType::Int, expected_literal: String::from("12")},
           Case {expected_type: TokenType::Equality, expected_literal: String::from("==")},
           Case {expected_type: TokenType::NotEqual, expected_literal: String::from("!=")},
           Case {expected_type: TokenType::Bang, expected_literal: String::from("!")},
           Case {expected_type: TokenType::Minus, expected_literal: String::from("-")},

           Case {expected_type: TokenType::Asterisk, expected_literal: String::from("*")},
           Case {expected_type: TokenType::Slash, expected_literal: String::from("/")},
           Case {expected_type: TokenType::Gt, expected_literal: String::from(">")},
           Case {expected_type: TokenType::Lt, expected_literal: String::from("<")},
       ];

       let mut lexer = Lexer::new(String::from(input));
       for case in cases {
           let tok = lexer.next_token();
           assert_eq!(tok.t_type, case.expected_type);
           assert_eq!(tok.literal, case.expected_literal);
       }
    }
}
