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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_token() {
        let token = Token::new(
            TokenType::ID,
            "foo".to_string(),
            Position {
                start: 0,
                end: 3,
                line: 1,
                column: 1,
            },
        );

        assert_eq!(token.token_type, TokenType::ID);
        assert_eq!(token.literal, "foo");
        assert_eq!(token.position.start, 0);
        assert_eq!(token.position.end, 3);
        assert_eq!(token.position.line, 1);
        assert_eq!(token.position.column, 1);
    }

    #[test]
    fn test_token_to_string() {
        let token = Token::new(
            TokenType::ID,
            "foo".to_string(),
            Position {
                start: 0,
                end: 3,
                line: 1,
                column: 1,
            },
        );

        assert_eq!(token.to_string(), "<ID:foo [Position { start: 0, end: 3, line: 1, column: 1 }]>");
    }

    #[test]
    fn test_new_position() {
        let position = Position::new();

        assert_eq!(position.start, 0);
        assert_eq!(position.end, 0);
        assert_eq!(position.line, 1);
        assert_eq!(position.column, 1);
    }
}