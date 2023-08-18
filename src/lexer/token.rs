use crate::lexer::token_type::TokenType;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Position {
    pub start: u32,
    pub end: u32,
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn new() -> Position {
        Position {
            start: 0,
            end: 0,
            line: 1,
            column: 1,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
        format!("<{:?}:{} [{:?}]>", self.token_type, self.literal, self.position)
    }
}