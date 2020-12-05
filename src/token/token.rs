#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum TokenType {
    Illegal,
    EOF,
    Ident,
    Int,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Equality,
    NotEqual,
    Gt,
    Lt,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Token {
    pub t_type: TokenType,
    pub literal: String,
}

