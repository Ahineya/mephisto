#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    LCURLY,
    RCURLY,
    LPAREN,
    RPAREN,
    LSQUARE,
    RSQUARE,
    SEMI,
    COLON,
    DOT,
    COMMA,
    NUMBER,
    STRING,

    DEF,
    DIV,
    MINUS,
    PLUS,
    MUL,
    GT,
    LT,
    GE,
    LE,
    EQ,
    NE,

    PROCESS,
    RETURN,
    BLOCK,
    PARAM,
    INPUT,
    OUTPUT,
    LET,
    CONST,
    BUFFER,

    FN,

    IMPORT,
    EXPORT,
    FROM,

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
