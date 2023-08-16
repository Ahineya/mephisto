use crate::lexer::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Position {
    pub start: u32,
    pub end: u32,
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Clone)]
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