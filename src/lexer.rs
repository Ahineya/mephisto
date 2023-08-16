mod tokenizers;
pub mod token;
pub mod token_type;

use regex::Regex;
use crate::lexer::token::{Position, Token};
use crate::lexer::token_type::TokenType;
use crate::lexer::tokenizers::{full_pattern_t, match_word_t, skip_whitespaces_t};

pub struct Lexer {
    tokenizers: Vec<fn(&str, u32) -> (Option<Token>, u32)>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokenizers: vec![
                skip_whitespaces_t,
                |chars: &str, current: u32| full_pattern_t(TokenType::COMMENT, Regex::new(r"^//.*").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::COMMENT, Regex::new(r"^/\*[^*]*\*+(?:[^/*][^*]*\*+)*/").unwrap(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::PROCESS, "process".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::BLOCK, "block".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::RETURN, "return".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::INPUT, "input".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::PARAM, "param".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::OUTPUT, "output".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::OUTPUTS, "OUTPUTS".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::LET, "let".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::CONST, "const".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::IMPORT, "import".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::EXPORT, "export".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::CONNECT, "connect".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::BUFFER, "buffer".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::LCURLY, "{".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::RCURLY, "}".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::LPAREN, "(".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::RPAREN, ")".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::LSQUARE, "[".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::RSQUARE, "]".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::CABLE, "->".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::SEMI, ";".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::COLON, ":".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::DOT, ".".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::COMMA, ",".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::EQ, "==".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::DEF, "=".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::DIV, "/".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::MUL, "*".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::MINUS, "-".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::PLUS, "+".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::GT, ">".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::LT, "<".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::GE, ">=".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::LE, "<=".to_string(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::NUMBER, Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::STRING, Regex::new(r#"^"([^"\\]|\\.)*""#).unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::ID, Regex::new(r"^_*[a-zA-Z][_a-zA-Z0-9]*").unwrap(), chars, current),
            ]
        }
    }

    pub fn tokenize(&self, input: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut position = Position { line: 1, column: 1, start: 0, end: 0 };

        let orig_str = input.clone();

        while position.start < input.len() as u32 {
            let mut tokenized = false;

            // println!("Current position: {}", position.start);

            for tokenizer in &self.tokenizers {
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
                let mut t = Token::new(TokenType::UNKNOWN, "".to_string(), position.clone());
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

        for token in &tokens {
            if token.token_type == TokenType::COMMENT {
                continue;
            }
            println!("{}", token.to_string());
        }

        tokens.retain(|t| t.token_type != TokenType::COMMENT);

        tokens
    }
}
