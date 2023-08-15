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
                |chars: &str, current: u32| full_pattern_t(TokenType::Comment, Regex::new(r"^//.*").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::Comment, Regex::new(r"^/\*[^*]*\*+(?:[^/*][^*]*\*+)*/").unwrap(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::Process, "process".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::Process, "return".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::LCurlyBrace, "{".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::RCurlyBrace, "}".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::Semi, ";".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::Dot, ".".to_string(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::Number, Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+").unwrap(), chars, current),
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

        for token in &tokens {
            if token.token_type == TokenType::Comment {
                continue;
            }
            println!("{}", token.to_string());
        }

        tokens
    }
}


