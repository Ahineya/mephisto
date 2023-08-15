use std::fmt;
use std::fmt::Formatter;
use crate::lexer::token_type::TokenType;

pub struct Position {
    pub start: u32,
    pub end: u32,
    pub line: u32,
    pub column: u32,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            line: self.line,
            column: self.column,
            start: self.start,
            end: self.end,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.start, self.end)
    }
}

pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub position: Position,
}

impl Token {
    pub fn new(token_type: TokenType, literal: String, position: Position) -> Token {
        Token {
            token_type,
            literal,
            position,
        }
    }

    pub fn to_string(&self) -> String {
        format!("<{}:{} [{}]>", self.token_type, self.literal, self.position)
    }
}