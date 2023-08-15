use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub enum TokenType {
    // Single-character tokens.
    // LeftParen, RightParen, LeftBrace, RightBrace,
    // Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    LCurlyBrace,
    RCurlyBrace,
    Semi,
    Dot,
    Number,
    //
    // // One or two character tokens.
    // Bang, BangEqual,
    // Equal, EqualEqual,
    // Greater, GreaterEqual,
    // Less, LessEqual,
    //
    // // Literals.
    // Identifier, String, Number,
    //
    // // Keywords.
    // And, Class, Else, False, Fun, For, If, Nil, Or,
    // Print, Return, Super, This, True, Var, While,

    Process,
    EOF,
    WS,

    Comment,

    Unknown,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LCurlyBrace => write!(f, "LCurlyBrace"),
            TokenType::RCurlyBrace => write!(f, "RCurlyBrace"),
            TokenType::Semi => write!(f, "Semi"),
            TokenType::Process => write!(f, "Process"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::WS => write!(f, "WS"),
            TokenType::Dot => write!(f, "Dot"),
            TokenType::Number => write!(f, "Number"),
            TokenType::Comment => write!(f, "Comment"),
            TokenType::Unknown => write!(f, "Unknown"),
        }
    }
}