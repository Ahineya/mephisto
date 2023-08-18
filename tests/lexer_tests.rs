#[cfg(test)]
mod tests {
    use mephisto::lexer::Lexer;
    use mephisto::lexer::token_type::TokenType;

    #[test]
    fn test_tokenize_lotion() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("".to_string());

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_unknown() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("import #".to_string());

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, TokenType::IMPORT);
        assert_eq!(tokens[1].token_type, TokenType::UNKNOWN);
        assert_eq!(tokens[2].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_import() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("import Math from \"./math.auo\";".to_string());

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].token_type, TokenType::IMPORT);
        assert_eq!(tokens[1].token_type, TokenType::ID);
        assert_eq!(tokens[2].token_type, TokenType::FROM);
        assert_eq!(tokens[3].token_type, TokenType::STRING);
        assert_eq!(tokens[4].token_type, TokenType::SEMI);
        assert_eq!(tokens[5].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_skip_comment() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("import Math from \"./math.auo\"; // this is a comment".to_string());

        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0].token_type, TokenType::IMPORT);
        assert_eq!(tokens[1].token_type, TokenType::ID);
        assert_eq!(tokens[2].token_type, TokenType::FROM);
        assert_eq!(tokens[3].token_type, TokenType::STRING);
        assert_eq!(tokens[4].token_type, TokenType::SEMI);
        assert_eq!(tokens[5].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_param() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("param frequency {
    min: 40;
    max: 22000;
}".to_string());

        assert_eq!(tokens.len(), 13);
        assert_eq!(tokens[0].token_type, TokenType::PARAM);
        assert_eq!(tokens[1].token_type, TokenType::ID);
        assert_eq!(tokens[2].token_type, TokenType::LCURLY);

        assert_eq!(tokens[3].token_type, TokenType::ID);
        assert_eq!(tokens[4].token_type, TokenType::COLON);
        assert_eq!(tokens[5].token_type, TokenType::NUMBER);
        assert_eq!(tokens[6].token_type, TokenType::SEMI);

        assert_eq!(tokens[7].token_type, TokenType::ID);
        assert_eq!(tokens[8].token_type, TokenType::COLON);
        assert_eq!(tokens[9].token_type, TokenType::NUMBER);
        assert_eq!(tokens[10].token_type, TokenType::SEMI);

        assert_eq!(tokens[11].token_type, TokenType::RCURLY);
        assert_eq!(tokens[12].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_vars() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("output out = 0;

let phase = 0;

const SR = 44100;

buffer b[1024] = |i| {
    return i / 1024;
};

input gain = 0;
".to_string());

        assert_eq!(tokens.len(), 36);

        // output out = 0;
        assert_eq!(tokens[0].token_type, TokenType::OUTPUT);
        assert_eq!(tokens[1].token_type, TokenType::ID);
        assert_eq!(tokens[2].token_type, TokenType::DEF);
        assert_eq!(tokens[3].token_type, TokenType::NUMBER);
        assert_eq!(tokens[4].token_type, TokenType::SEMI);

        // let phase = 0;
        assert_eq!(tokens[5].token_type, TokenType::LET);
        assert_eq!(tokens[6].token_type, TokenType::ID);
        assert_eq!(tokens[7].token_type, TokenType::DEF);
        assert_eq!(tokens[8].token_type, TokenType::NUMBER);
        assert_eq!(tokens[9].token_type, TokenType::SEMI);

        // const SR = 44100;
        assert_eq!(tokens[10].token_type, TokenType::CONST);
        assert_eq!(tokens[11].token_type, TokenType::ID);
        assert_eq!(tokens[12].token_type, TokenType::DEF);
        assert_eq!(tokens[13].token_type, TokenType::NUMBER);
        assert_eq!(tokens[14].token_type, TokenType::SEMI);

        // buffer b[1024] = |i| {
        //    return i / 1024;
        // };

        assert_eq!(tokens[15].token_type, TokenType::BUFFER);
        assert_eq!(tokens[16].token_type, TokenType::ID);
        assert_eq!(tokens[17].token_type, TokenType::LSQUARE);
        assert_eq!(tokens[18].token_type, TokenType::NUMBER);
        assert_eq!(tokens[19].token_type, TokenType::RSQUARE);
        assert_eq!(tokens[20].token_type, TokenType::DEF);
        assert_eq!(tokens[21].token_type, TokenType::BUFI);
        assert_eq!(tokens[22].token_type, TokenType::LCURLY);

        assert_eq!(tokens[23].token_type, TokenType::RETURN);
        assert_eq!(tokens[24].token_type, TokenType::ID);
        assert_eq!(tokens[25].token_type, TokenType::DIV);
        assert_eq!(tokens[26].token_type, TokenType::NUMBER);
        assert_eq!(tokens[27].token_type, TokenType::SEMI);

        assert_eq!(tokens[28].token_type, TokenType::RCURLY);
        assert_eq!(tokens[29].token_type, TokenType::SEMI);

        // input gain = 0;
        assert_eq!(tokens[30].token_type, TokenType::INPUT);
        assert_eq!(tokens[31].token_type, TokenType::ID);
        assert_eq!(tokens[32].token_type, TokenType::DEF);
        assert_eq!(tokens[33].token_type, TokenType::NUMBER);
        assert_eq!(tokens[34].token_type, TokenType::SEMI);

        assert_eq!(tokens[35].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_export() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("export const PI = 3.14;".to_string());

        assert_eq!(tokens.len(), 7);
        assert_eq!(tokens[0].token_type, TokenType::EXPORT);
        assert_eq!(tokens[1].token_type, TokenType::CONST);
        assert_eq!(tokens[2].token_type, TokenType::ID);
        assert_eq!(tokens[3].token_type, TokenType::DEF);
        assert_eq!(tokens[4].token_type, TokenType::NUMBER);
        assert_eq!(tokens[5].token_type, TokenType::SEMI);
        assert_eq!(tokens[6].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_function_declaration() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("getSaw(phase) {
    return phase * 2 - 1;
}".to_string());

        assert_eq!(tokens.len(), 14);
        assert_eq!(tokens[0].token_type, TokenType::ID);
        assert_eq!(tokens[1].token_type, TokenType::LPAREN);
        assert_eq!(tokens[2].token_type, TokenType::ID);
        assert_eq!(tokens[3].token_type, TokenType::RPAREN);
        assert_eq!(tokens[4].token_type, TokenType::LCURLY);
        assert_eq!(tokens[5].token_type, TokenType::RETURN);
        assert_eq!(tokens[6].token_type, TokenType::ID);
        assert_eq!(tokens[7].token_type, TokenType::MUL);
        assert_eq!(tokens[8].token_type, TokenType::NUMBER);
        assert_eq!(tokens[9].token_type, TokenType::MINUS);
        assert_eq!(tokens[10].token_type, TokenType::NUMBER);
        assert_eq!(tokens[11].token_type, TokenType::SEMI);
        assert_eq!(tokens[12].token_type, TokenType::RCURLY);
        assert_eq!(tokens[13].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_blocks() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("block {} process {} connect {}"
            .to_string());

        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].token_type, TokenType::BLOCK);
        assert_eq!(tokens[1].token_type, TokenType::LCURLY);
        assert_eq!(tokens[2].token_type, TokenType::RCURLY);
        assert_eq!(tokens[3].token_type, TokenType::PROCESS);
        assert_eq!(tokens[4].token_type, TokenType::LCURLY);
        assert_eq!(tokens[5].token_type, TokenType::RCURLY);
        assert_eq!(tokens[6].token_type, TokenType::CONNECT);
        assert_eq!(tokens[7].token_type, TokenType::LCURLY);
        assert_eq!(tokens[8].token_type, TokenType::RCURLY);
        assert_eq!(tokens[9].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_operators() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("a + b - c * d / e"
            .to_string());

        assert_eq!(tokens.len(), 10);
        assert_eq!(tokens[0].token_type, TokenType::ID);
        assert_eq!(tokens[1].token_type, TokenType::PLUS);
        assert_eq!(tokens[2].token_type, TokenType::ID);
        assert_eq!(tokens[3].token_type, TokenType::MINUS);
        assert_eq!(tokens[4].token_type, TokenType::ID);
        assert_eq!(tokens[5].token_type, TokenType::MUL);
        assert_eq!(tokens[6].token_type, TokenType::ID);
        assert_eq!(tokens[7].token_type, TokenType::DIV);
        assert_eq!(tokens[8].token_type, TokenType::ID);
        assert_eq!(tokens[9].token_type, TokenType::EOF);
    }

    #[test]
    fn test_tokenize_logical_operators() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("a > b < c <= d >= e == f != g"
            .to_string());

        assert_eq!(tokens.len(), 14);
        assert_eq!(tokens[0].token_type, TokenType::ID);
        assert_eq!(tokens[1].token_type, TokenType::GT);
        assert_eq!(tokens[2].token_type, TokenType::ID);
        assert_eq!(tokens[3].token_type, TokenType::LT);
        assert_eq!(tokens[4].token_type, TokenType::ID);
        assert_eq!(tokens[5].token_type, TokenType::LE);
        assert_eq!(tokens[6].token_type, TokenType::ID);
        assert_eq!(tokens[7].token_type, TokenType::GE);
        assert_eq!(tokens[8].token_type, TokenType::ID);
        assert_eq!(tokens[9].token_type, TokenType::EQ);
        assert_eq!(tokens[10].token_type, TokenType::ID);
        assert_eq!(tokens[11].token_type, TokenType::NE);
        assert_eq!(tokens[12].token_type, TokenType::ID);
        assert_eq!(tokens[13].token_type, TokenType::EOF);
    }

    #[test]
    fn tokenize_connections() {
        let tokenizer = Lexer::new();

        let tokens = tokenizer.tokenize("connect {
    out -> OUTPUTS[0];
    phase -> Kick.phase;
    Kick.out -> kick;
}"
            .to_string());

        assert_eq!(tokens.len(), 23);
        assert_eq!(tokens[0].token_type, TokenType::CONNECT);
        assert_eq!(tokens[1].token_type, TokenType::LCURLY);

        assert_eq!(tokens[2].token_type, TokenType::ID);
        assert_eq!(tokens[3].token_type, TokenType::CABLE);
        assert_eq!(tokens[4].token_type, TokenType::OUTPUTS);
        assert_eq!(tokens[5].token_type, TokenType::LSQUARE);
        assert_eq!(tokens[6].token_type, TokenType::NUMBER);
        assert_eq!(tokens[7].token_type, TokenType::RSQUARE);
        assert_eq!(tokens[8].token_type, TokenType::SEMI);

        assert_eq!(tokens[9].token_type, TokenType::ID);
        assert_eq!(tokens[10].token_type, TokenType::CABLE);
        assert_eq!(tokens[11].token_type, TokenType::ID);
        assert_eq!(tokens[12].token_type, TokenType::DOT);
        assert_eq!(tokens[13].token_type, TokenType::ID);
        assert_eq!(tokens[14].token_type, TokenType::SEMI);

        assert_eq!(tokens[15].token_type, TokenType::ID);
        assert_eq!(tokens[16].token_type, TokenType::DOT);
        assert_eq!(tokens[17].token_type, TokenType::ID);
        assert_eq!(tokens[18].token_type, TokenType::CABLE);
        assert_eq!(tokens[19].token_type, TokenType::ID);
        assert_eq!(tokens[20].token_type, TokenType::SEMI);

        assert_eq!(tokens[21].token_type, TokenType::RCURLY);
        assert_eq!(tokens[22].token_type, TokenType::EOF);

        // Kick in "phase -> Kick.phase"
        assert_eq!(tokens[11].position.line, 3);
        assert_eq!(tokens[11].position.column, 12);
        assert_eq!(tokens[11].position.start, 46);
        assert_eq!(tokens[11].position.end, 50);
    }
}
