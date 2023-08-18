pub mod lexer;
pub mod parser;
pub mod symbol_table;

use crate::lexer::{token::{Token}, Lexer};
use crate::parser::{Parser};
use crate::parser::ast::AST;
use crate::symbol_table::SymbolTable;

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

    pub fn parse(&self, tokens: Vec<Token>) -> AST {
        println!("Input tokens: {:#?}", tokens);
        println!();
        println!("Mephisto is parsing...");
        println!();
        println!("AST:");

        let mut parser = Parser::new();
        parser.parse(tokens)
    }

    pub fn create_symbol_table(&self, ast: &mut AST) -> Result<SymbolTable, Vec<String>> {
        println!("Input AST: {:#?}", ast);
        println!();
        println!("Mephisto is creating symbol table...");
        println!();
        println!("Symbol table:");

        SymbolTable::from_ast(ast)
    }

    pub fn validate_semantics(&self, ast: &mut AST, symbol_table: &mut SymbolTable) -> Result<String, String> {
        println!("Input symbol table: {:#?}", symbol_table);
        println!();
        println!("Mephisto is validating semantics...");
        println!();

        // Err("Not implemented".to_string())

        todo!("Validating semantics")
    }

    pub fn compile(&self, input: String) -> Result<String, Vec<String>> {
        let tokens = self.tokenize(input);
        let mut ast = self.parse(tokens);

        if ast.errors.len() > 0 {
            return Err(ast.errors);
        }

        println!("{}", ast.to_json());

        let symbol_table = self.create_symbol_table(&mut ast);

        match symbol_table {
            Ok(mut symbol_table) => {
                match self.validate_semantics(&mut ast, &mut symbol_table) {
                    Ok(_) => {},
                    Err(errors) => {
                        return Err(vec![errors]);
                    }
                }

                todo!("Compiling")
            },
            Err(errors) => {
                return Err(errors);
            }
        }
    }
}
