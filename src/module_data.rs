use crate::lexer::token::Position;
use crate::parser::ast::{AST, Node};
use crate::symbol_table::SymbolTable;

#[derive(Debug, Clone)]
pub struct ModuleData {
    pub ast: AST,
    pub symbol_table: SymbolTable,
    pub errors: Vec<String>,
}

impl ModuleData {
    pub fn new() -> Self {
        ModuleData {
            ast: AST::new(Node::ProgramNode { children: vec![], position: Position::new() }, vec![]),
            symbol_table: SymbolTable::new(),
            errors: vec![],
        }
    }
}
