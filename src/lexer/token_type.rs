use std::fmt;
use std::fmt::Formatter;

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
    // LeftParen, RightParen, LeftBrace, RightBrace,
    // Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    LCURLY,
    RCURLY,
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

    DEF,
    DIV, MINUS, PLUS, MUL,
    GT, LT, GE, LE,
    EQ,

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
    BUFFER,

    IMPORT, EXPORT, FROM,

    CONNECT,
    CABLE,

    OUTPUTS,

    ID,

    EOF,
    WS,

    COMMENT,

    UNKNOWN,

    BUFI,
}
