mod lexer;
mod parser;

use std::fmt;
use std::fmt::Formatter;

use regex::Regex;
use crate::lexer::{token::{Token}, Lexer};
use crate::parser::{AST, Parser};

fn main() {
    println!("Hello, world!");
}

pub struct Mephisto {}


impl Mephisto {
    pub fn new() -> Mephisto {
        Mephisto {}
    }

    pub fn tokenize(&self, input: String) -> Vec<Token> {
        println!("Input string: {}", input);
        println!();
        println!("Mephisto is tokenizing...");
        println!();
        println!("Tokens:");

        let lexer = Lexer::new();
        lexer.tokenize(input)
    }

    pub fn parse(&self, input: Vec<Token>) -> AST {
        println!("Input string: {:?}", input);
        println!();
        println!("Mephisto is parsing...");
        println!();
        println!("AST:");

        let mut parser = Parser::new(input);
        parser.parse()
    }
}
