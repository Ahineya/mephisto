use regex::Regex;

use crate::lexer::token::{Position, Token};
use crate::lexer::token_type::TokenType;

fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n'
}

pub fn skip_whitespaces_t(chars: &str, current: u32) -> (Option<Token>, u32, u32, u32) {
    let mut consumed_chars = 0;

    let mut skipped_lines = 0;
    let mut skipped_columns = 0;

    // From current position, skip all whitespaces
    // start from "current" position
    for c in chars.chars().skip(current as usize) {
        if !is_whitespace(c) {
            break;
        }

        if c == '\n' {
            skipped_lines += 1;
            skipped_columns = 0;
        } else {
            skipped_columns += 1;
        }

        consumed_chars += 1;
    }

    (None, consumed_chars, skipped_lines, skipped_columns)
}

pub fn full_pattern_t(token_type: TokenType, pattern: Regex, chars: &str, current: u32) -> (Option<Token>, u32, u32, u32) {
    let str = chars.to_string();


    // Create new string from the current position
    let str = str.chars().skip(current as usize).collect::<String>();

    let p = pattern.captures(str.as_str());

    if let Some(p) = p {
        let matched = p.get(0).unwrap().as_str().to_string();
        let consumed_chars = matched.len() as u32;

        // Count the number of lines and columns skipped

        let mut skipped_lines = 0;
        let mut skipped_columns = 0;

        for c in matched.chars() {
            if c == '\n' {
                skipped_lines += 1;
                skipped_columns = 0;
            } else {
                skipped_columns += 1;
            }
        }

        return (Some(Token::new(token_type, matched, Position { line: 0, column: 0, start: 0, end: 0 })), consumed_chars, skipped_lines, skipped_columns);
    }

    (None, 0, 0, 0)
}

pub fn match_word_t(token_type: TokenType, word: String, chars: &str, current: u32) -> (Option<Token>, u32, u32, u32) {
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
        (Some(Token::new(token_type, value, Position { line: 0, column: 0, start: 0, end: 0 })), consumed_chars, 0, consumed_chars)
    } else {
        (None, 0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::token_type::TokenType;
    use crate::lexer::tokenizers::{full_pattern_t, match_word_t, skip_whitespaces_t};

    #[test]
    fn test_skip_whitespaces_t() {
        let (token, consumed_chars, skipped_lines, skipped_columns) = skip_whitespaces_t("   \n\t  ", 0);

        assert_eq!(consumed_chars, 7);
        assert_eq!(token, None);
        assert_eq!(skipped_lines, 1);
        assert_eq!(skipped_columns, 3);
    }

    #[test]
    fn test_full_pattern_t() {
        let (token, consumed_chars, skipped_lines, skipped_columns) = full_pattern_t(TokenType::NUMBER, regex::Regex::new(r"^\d+").unwrap(), "123", 0);

        assert_eq!(consumed_chars, 3);
        assert_eq!(token.unwrap().literal, "123");
        assert_eq!(skipped_lines, 0);
        assert_eq!(skipped_columns, 3);
    }

    #[test]
    fn test_match_word_t() {
        let (token, consumed_chars, skipped_lines, skipped_columns) = match_word_t(TokenType::ID, "hello".to_string(), "hello", 0);

        assert_eq!(consumed_chars, 5);
        assert_eq!(token.unwrap().literal, "hello");
        assert_eq!(skipped_lines, 0);
        assert_eq!(skipped_columns, 5);
    }

    #[test]
    fn test_match_word_t_with_extra_chars() {
        let (token, consumed_chars, skipped_lines, skipped_columns) = match_word_t(TokenType::ID, "hello".to_string(), "hello[]", 0);

        assert_eq!(consumed_chars, 5);
        assert_eq!(token.unwrap().literal, "hello");
        assert_eq!(skipped_lines, 0);
        assert_eq!(skipped_columns, 5);
    }

    #[test]
    fn test_skip_multiline_comments() {
        let (token, consumed_chars, skipped_lines, skipped_columns) = full_pattern_t(TokenType::COMMENT, regex::Regex::new(r"^/\*[^*]*\*+(?:[^/*][^*]*\*+)*/").unwrap(), "/* hello
world
*/
        hey", 0);

        assert_eq!(consumed_chars, 17);
        assert_eq!(token.unwrap().literal, "/* hello
world
*/");

        assert_eq!(skipped_lines, 2);
        assert_eq!(skipped_columns, 2);
    }
}
