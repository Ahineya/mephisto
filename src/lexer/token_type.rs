use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug)]
pub enum TokenType {
    // Single-character tokens.
    // LeftParen, RightParen, LeftBrace, RightBrace,
    // Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    LCURL,
    RCURL,
    LPAREN, RPAREN,
    LSQUARE, RSQUARE,
    SEMI,
    COLON,
    DOT,
    COMMA,
    NUMBER,
    STRING,
    //
    // // One or two character tokens.
    // Bang, BangEqual,
    // Equal, EqualEqual,
    // Greater, GreaterEqual,
    // Less, LessEqual,

    EQ, DIV, MINUS, PLUS, MUL,
    GT, LT, GE, LE,

    //
    // // Literals.
    // Identifier, String, Number,
    //
    // // Keywords.
    // And, Class, Else, False, Fun, For, If, Nil, Or,
    // Print, Return, Super, This, True, Var, While,

    PROCESS,
    RETURN,
    BLOCK,
    PARAM,
    INPUT,
    OUTPUT,
    LET,
    CONST,

    IMPORT, EXPORT,

    CONNECT,
    CABLE,

    ID,

    EOF,
    WS,

    COMMENT,

    UNKNOWN,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::LCURL => write!(f, "LCURL"),
            TokenType::RCURL => write!(f, "RCURL"),
            TokenType::SEMI => write!(f, "SEMI"),
            TokenType::PROCESS => write!(f, "PROCESS"),
            TokenType::RETURN => write!(f, "RETURN"),
            TokenType::BLOCK => write!(f, "BLOCK"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::WS => write!(f, "WS"),
            TokenType::EQ => write!(f, "EQ"),
            TokenType::DIV => write!(f, "DIV"),
            TokenType::DOT => write!(f, "Dot"),
            TokenType::NUMBER => write!(f, "NUMBER"),
            TokenType::COMMENT => write!(f, "COMMENT"),
            TokenType::ID => write!(f, "ID"),
            TokenType::UNKNOWN => write!(f, "UNKNOWN"),
            TokenType::INPUT => write!(f, "INPUT"),
            TokenType::OUTPUT => write!(f, "OUTPUT"),
            TokenType::LET => write!(f, "LET"),
            TokenType::CONST => write!(f, "CONST"),
            TokenType::PARAM => write!(f, "PARAM"),
            TokenType::COLON => write!(f, "COLON"),
            TokenType::COMMA => write!(f, "COMMA"),
            TokenType::MINUS => write!(f, "MINUS"),
            TokenType::PLUS => write!(f, "PLUS"),
            TokenType::MUL => write!(f, "STAR"),
            TokenType::GT => write!(f, "GT"),
            TokenType::LT => write!(f, "LT"),
            TokenType::GE => write!(f, "GE"),
            TokenType::LE => write!(f, "LE"),
            TokenType::LPAREN => write!(f, "LPAREN"),
            TokenType::RPAREN => write!(f, "RPAREN"),
            TokenType::IMPORT => write!(f, "IMPORT"),
            TokenType::EXPORT => write!(f, "EXPORT"),

            TokenType::LSQUARE => write!(f, "LSQUARE"),
            TokenType::RSQUARE => write!(f, "RSQUARE"),

            TokenType::CONNECT => write!(f, "CONNECT"),
            TokenType::CABLE => write!(f, "CABLE"),

            TokenType::STRING => write!(f, "STRING"),
        }
    }
}