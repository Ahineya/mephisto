use indexmap::IndexMap;
use crate::module_data::ModuleData;
use crate::parser::ast::AST;
use crate::symbol_table::SymbolTable;

pub struct IR {
    pub errors: Vec<String>,
}

#[derive(Debug)]
pub struct IRResult {
    pub ast: AST,
    pub symbol_table: SymbolTable,
    pub errors: Vec<String>,
}

impl IR {
    pub fn new() -> IR {
        IR {
            errors: vec![],
        }
    }

    pub fn create(&mut self, modules: &mut IndexMap<String, ModuleData>, main_module: String) -> Result<IRResult, Vec<String>> {
        todo!("Implement IR creation")
    }
}