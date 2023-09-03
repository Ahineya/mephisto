mod tokenizers;
pub mod token;
pub mod token_type;

use regex::Regex;
use crate::lexer::token::{Position, Token};
use crate::lexer::token_type::TokenType;
use crate::lexer::tokenizers::{full_pattern_t, match_word_t, skip_whitespaces_t};

pub struct Lexer {
    tokenizers: Vec<fn(&str, u32) -> (Option<Token>, u32, u32, u32)>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            tokenizers: vec![
                skip_whitespaces_t,
                |chars: &str, current: u32| full_pattern_t(TokenType::COMMENT, Regex::new(r"^//.*").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::COMMENT, Regex::new(r"^/\*[^*]*\*+(?:[^/*][^*]*\*+)*/").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::PROCESS, Regex::new(r"^process\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::FN, Regex::new(r"^fn\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::BLOCK, Regex::new(r"^block\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::RETURN, Regex::new(r"^return\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::INPUT, Regex::new(r"^input\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::PARAM, Regex::new(r"^param\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::OUTPUT, Regex::new(r"^output\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::OUTPUTS, Regex::new(r"^OUTPUTS\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::LET, Regex::new(r"^let\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::CONST, Regex::new(r"^const\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::IMPORT, Regex::new(r"^import\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::FROM, Regex::new(r"^from\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::EXPORT, Regex::new(r"^export\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::CONNECT, Regex::new(r"^connect\b").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::BUFFER, Regex::new(r"^buffer\b").unwrap(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::EQ, "==".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::NE, "!=".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::GE, ">=".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::LE, "<=".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::BUFI, "|i|".to_string(), chars, current),
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
                |chars: &str, current: u32| match_word_t(TokenType::DEF, "=".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::DIV, "/".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::MUL, "*".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::MINUS, "-".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::PLUS, "+".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::GT, ">".to_string(), chars, current),
                |chars: &str, current: u32| match_word_t(TokenType::LT, "<".to_string(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::NUMBER, Regex::new(r"^[+-]?([0-9]*[.])?[0-9]+").unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::STRING, Regex::new(r#"^"([^"\\]|\\.)*""#).unwrap(), chars, current),
                |chars: &str, current: u32| full_pattern_t(TokenType::ID, Regex::new(r"^[_$]*[_$a-zA-Z][$_a-zA-Z0-9]*").unwrap(), chars, current),
            ]
        }
    }

    pub fn tokenize(&self, input: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut position = Position { line: 1, column: 1, start: 0, end: 0 };

        let orig_str = input.clone();

        while position.start < input.len() as u32 {
            let mut tokenized = false;

            for tokenizer in &self.tokenizers {
                let (token, consumed, skipped_lines, skipped_columns) = tokenizer(&orig_str, position.start);

                // TODO: Some bullshit here
                if consumed > 0 {
                    tokenized = true;
                    position.start += consumed;
                    position.end += consumed;
                    position.line += skipped_lines;
                    position.column = if skipped_lines > 0 { skipped_columns + 1 } else {position.column};
                }

                if let Some(mut t) = token {
                    t.position.start = position.start - consumed;
                    t.position.end = position.start;
                    t.position.line += position.line - skipped_lines;
                    t.position.column = position.column;

                    position.column += consumed;

                    tokens.push(t);
                }
            }

            if !tokenized {
                let mut t = Token::new(TokenType::UNKNOWN, "".to_string(), position.clone());

                let char = input.chars().nth(position.start as usize);

                if char.is_none() {
                    println!("{:#?}", position);
                    panic!("Unexpected who knows what happened")
                }

                t.literal = input.chars().nth(position.start as usize).unwrap().to_string();
                t.position.start = position.start;
                t.position.end = position.start + 1;
                t.position.column += 1;
                position.start += 1;

                tokens.push(t);
                break;
            }
        }

        tokens.push(Token::new(TokenType::EOF, "".to_string(), position));

        for token in &tokens {
            if token.token_type == TokenType::COMMENT {
                continue;
            }
        }

        tokens.retain(|t| t.token_type != TokenType::COMMENT);

        tokens
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_comments() {
        let lexer = super::Lexer::new();
        let tokens = lexer.tokenize("// This is a comment".to_string());

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, super::token_type::TokenType::EOF);
    }

    #[test]
    fn test_import_comment() {
        let lexer = super::Lexer::new();
        let tokens = lexer.tokenize("import Limiter from \"./limiter.mephisto\";
        //import Freeverb from \"./freeverb.mephisto\";".to_string());

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].token_type, super::token_type::TokenType::IMPORT);
        assert_eq!(tokens[1].token_type, super::token_type::TokenType::ID);
        assert_eq!(tokens[2].token_type, super::token_type::TokenType::FROM);
        assert_eq!(tokens[3].token_type, super::token_type::TokenType::STRING);
        assert_eq!(tokens[4].token_type, super::token_type::TokenType::SEMI);
        assert_eq!(tokens[5].token_type, super::token_type::TokenType::EOF);
    }
}
