use regex::Regex;

use crate::lexer::token::{Position, Token};
use crate::lexer::token_type::TokenType;

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

pub fn skip_whitespaces_t(chars: &str, current: u32) -> (Option<Token>, u32) {
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

pub fn pattern_t(token_type: TokenType, pattern: Regex, chars: &str, current: u32) -> (Option<Token>, u32) {
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

pub fn full_pattern_t(token_type: TokenType, pattern: Regex, chars: &str, current: u32) -> (Option<Token>, u32) {
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

pub fn match_word_t(token_type: TokenType, word: String, chars: &str, current: u32) -> (Option<Token>, u32) {
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