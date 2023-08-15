use std::fmt;
use std::fmt::Formatter;

use regex::Regex;

fn main() {
    println!("Hello, world!");
}

enum TokenType {
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

impl PartialEq for TokenType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            TokenType::LCurlyBrace => {
                match other {
                    TokenType::LCurlyBrace => true,
                    _ => false,
                }
            }
            TokenType::RCurlyBrace => {
                match other {
                    TokenType::RCurlyBrace => true,
                    _ => false,
                }
            }
            TokenType::Semi => {
                match other {
                    TokenType::Semi => true,
                    _ => false,
                }
            }
            TokenType::Process => {
                match other {
                    TokenType::Process => true,
                    _ => false,
                }
            }
            TokenType::EOF => {
                match other {
                    TokenType::EOF => true,
                    _ => false,
                }
            }
            TokenType::WS => {
                match other {
                    TokenType::WS => true,
                    _ => false,
                }
            }
            TokenType::Dot => {
                match other {
                    TokenType::Dot => true,
                    _ => false,
                }
            }
            TokenType::Number => {
                match other {
                    TokenType::Number => true,
                    _ => false,
                }
            }
            TokenType::Comment => {
                match other {
                    TokenType::Comment => true,
                    _ => false,
                }
            }
            TokenType::Unknown => {
                match other {
                    TokenType::Unknown => true,
                    _ => false,
                }
            }
        }
    }
}

struct Position {
    // line: u32,
    // column: u32,
    start: u32,
    end: u32,
    line: u32,
    column: u32,
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

struct Token {
    token_type: TokenType,
    literal: String,
    position: Position,
}

impl Token {
    fn new(token_type: TokenType, literal: String, position: Position) -> Token {
        Token {
            token_type,
            literal,
            position,
        }
    }

    fn to_string(&self) -> String {
        format!("<{}:{} [{}]>", self.token_type, self.literal, self.position)
    }
}

pub struct Mephisto {}

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

fn skip_whitespaces_t(chars: &str, current: u32) -> (Option<Token>, u32) {
    let mut consumed_chars = 0;

    // From current position, skip all whitespaces
    // start from "current" position
    for c in chars.chars().skip(current as usize) {
        if !is_whitespace(c) {
            break;
        }

        consumed_chars += 1;
    }

    (None, consumed_chars)
}

fn pattern_t(token_type: TokenType, pattern: Regex, chars: &str, current: u32) -> (Option<Token>, u32) {
    let mut consumed_chars = 0;
    let mut value = String::new();

    let mut chars = chars.chars().skip(current as usize);

    while let Some(c) = chars.next() {
        if !pattern.is_match(&c.to_string()) {
            break;
        }

        value.push(c);
        consumed_chars += 1;
    }

    if value.len() == 0 {
        return (None, consumed_chars);
    }

    (Some(Token::new(token_type, value, Position { line: 0, column: 0, start: 0, end: 0 })), consumed_chars)
}

fn full_pattern_t(token_type: TokenType, pattern: Regex, chars: &str, current: u32) -> (Option<Token>, u32) {
    let mut consumed_chars = 0;

    let str = chars.to_string();

    // Create new string from the current position
    let str = str.chars().skip(current as usize).collect::<String>();

    let p = pattern.captures(str.as_str());

    if let Some(p) = p {
        let matched = p.get(0).unwrap().as_str().to_string();
        consumed_chars = matched.len() as u32;
        return (Some(Token::new(token_type, matched, Position { line: 0, column: 0, start: 0, end: 0 })), consumed_chars);
    }

    (None, 0)
}

fn match_word_t(token_type: TokenType, word: String, chars: &str, current: u32) -> (Option<Token>, u32) {
    let mut consumed_chars = 0;
    let mut value = String::new();

    let word_len = word.len();

    // Iterate over the "chars" starting from the current position

    let mut chars = chars.chars().skip(current as usize);

    while let Some(c) = chars.next() {
        if value.len() == word_len {
            break;
        }

        value.push(c);

        consumed_chars += 1;
    }

    if value.eq(&word) {
        (Some(Token::new(token_type, value, Position { line: 0, column: 0, start: 0, end: 0 })), consumed_chars)
    } else {
        (None, 0)
    }
}

impl Mephisto {
    pub fn new() -> Mephisto {
        Mephisto {}
    }

    pub fn tokenize(&self, input: String) {
        println!("Input string: {}", input);
        println!();
        println!("Mephisto is tokenizing...");
        println!();
        println!("Tokens:");

        let tokenizers: Vec<fn(&str, u32) -> (Option<Token>, u32)> = vec![
            skip_whitespaces_t,
            |chars: &str, current: u32| full_pattern_t(TokenType::Comment, Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+").unwrap(), chars, current),
            |chars: &str, current: u32| match_word_t(TokenType::Process, "process".to_string(), chars, current),
            |chars: &str, current: u32| match_word_t(TokenType::Process, "return".to_string(), chars, current),
            |chars: &str, current: u32| match_word_t(TokenType::LCurlyBrace, "{".to_string(), chars, current),
            |chars: &str, current: u32| match_word_t(TokenType::RCurlyBrace, "}".to_string(), chars, current),
            |chars: &str, current: u32| match_word_t(TokenType::Semi, ";".to_string(), chars, current),
            |chars: &str, current: u32| match_word_t(TokenType::Dot, ".".to_string(), chars, current),
            |chars: &str, current: u32| full_pattern_t(TokenType::Number, Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+").unwrap(), chars, current),
            |chars: &str, current: u32| full_pattern_t(TokenType::Comment, Regex::new(r"^//.*").unwrap(), chars, current),
            |chars: &str, current: u32| full_pattern_t(TokenType::Comment, Regex::new(r"^/\*[^*]*\*+(?:[^/*][^*]*\*+)*/").unwrap(), chars, current),
        ];

        let mut tokens: Vec<Token> = Vec::new();
        let mut position = Position { line: 1, column: 1, start: 0, end: 0 };

        let orig_str = input.clone();

        while position.start < input.len() as u32 {
            let mut tokenized = false;

            // println!("Current position: {}", position.start);

            for tokenizer in &tokenizers {
                let (token, consumed) = tokenizer(&orig_str, position.start);

                if consumed > 0 {
                    tokenized = true;
                    position.start += consumed;
                }

                if let Some(mut t) = token {
                    t.position.start = position.start - consumed;
                    t.position.end = position.start;
                    tokens.push(t);
                }
            }

            if !tokenized {
                println!("Unknown token at position {}", position.start);
                let mut t = Token::new(TokenType::Unknown, "".to_string(), position.clone());
                t.literal = input.chars().nth(position.start as usize).unwrap().to_string();
                t.position.start = position.start;
                t.position.end = position.start + 1;

                tokens.push(t);
                for token in tokens {
                    println!("{}", token.to_string());
                }
                panic!("Unknown token");
            }
        }

        tokens.push(Token::new(TokenType::EOF, "".to_string(), position.clone()));

        for token in tokens {
            if token.token_type == TokenType::Comment {
                continue;
            }
            println!("{}", token.to_string());
        }
    }
}
