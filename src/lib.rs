use std::error::Error;
use indexmap::IndexMap;
use crate::ir::IR;

use crate::lexer::{Lexer, token::Token};
use crate::module_data::ModuleData;
use crate::module_loader::{FileLoader, StubFileLoader};
use crate::parser::ast::{AST};
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::symbol_table::SymbolTable;

pub mod lexer;
pub mod parser;
pub mod symbol_table;
pub mod semantic;

pub mod module_data;

pub mod module_loader;

pub mod ir;

pub struct Mephisto<T: FileLoader> {
    loader: T,
}

#[derive(Debug)]
struct Context {
    loaded_modules: Box<Vec<String>>,
    modules: Box<IndexMap<String, ModuleData>>,
}

impl Mephisto<StubFileLoader> {
    pub fn tokenize(input: String) -> Vec<Token> {
        // println!("Input string: {}", input);
        // println!();
        // println!("Mephisto is tokenizing...");
        // println!();
        // println!("Tokens:");

        let lexer = Lexer::new();
        lexer.tokenize(input)
    }

    pub fn parse(tokens: Vec<Token>) -> AST {
        // println!("Input tokens: {:#?}", tokens);
        // println!();
        // println!("Mephisto is parsing...");
        // println!();
        // println!("AST:");

        let mut parser = Parser::new();
        parser.parse(tokens)
    }

    pub fn create_symbol_table(ast: &mut AST) -> Result<SymbolTable, Vec<String>> {
        // println!("Input AST: {:#?}", ast);
        // println!();
        // println!("Mephisto is creating symbol table...");
        // println!();
        // println!("Symbol table:");

        SymbolTable::from_ast(ast)
    }

    pub fn create_ir(modules: &mut Box<IndexMap<String, ModuleData>>, main_module: String) -> Result<ir::IRResult, Vec<String>> {
        let mut ir = IR::new();
        ir.create(&mut *modules, main_module)
    }
}

impl<T: FileLoader> Mephisto<T> {
    pub fn new(loader: T) -> Self {
        Mephisto {
            loader,
        }
    }

    pub fn validate_semantics(&self, modules: &mut IndexMap<String, ModuleData>) -> Result<String, Vec<String>> {
        let mut semantic = SemanticAnalyzer::new();
        semantic.validate_semantics(modules)
    }

    pub fn compile(&mut self, main_module_path: &str) -> Result<String, Vec<String>> {
        let modules: IndexMap<String, ModuleData> = IndexMap::new();

        let mut context = Context {
            loaded_modules: Box::new(Vec::new()),
            modules: Box::new(modules),
        };

        self.process_module(main_module_path, &mut context)?; // Recursively process all modules

        println!("Modules: {:#?}", context);

        let mut modules = context.modules;

        let main_module = modules.get(main_module_path);

        if main_module.is_none() {
            return Err(vec![format!("Main module {} not found", main_module_path)]);
        }

        self.validate_semantics(&mut modules)?;


        let main_module = modules.get_mut(main_module_path).unwrap();

        let mut code = main_module.ast.to_code_string();

        code.push_str("\n\n");

        println!("Code: {}", code);
        let mut ir = IR::new();
        let ir_result = ir.create(&mut modules, main_module_path.to_string())?;

        println!("IR: {:#?}", ir_result);

        todo!("Compiling")
    }

    fn process_module(&mut self, path: &str, context: &mut Context) -> Result<(), Vec<String>> {
        if context.loaded_modules.contains(&path.to_string()) {
            return Ok(());
        }

        let mut module = ModuleData::new();

        let input = self.load_module(path);

        if input.is_err() {
            module.errors.push(input.err().unwrap().to_string());
            context.modules.insert(path.to_string(), module);

            return Ok(());
        }

        let input = input.unwrap();

        context.loaded_modules.push(path.to_string());

        let tokens = Mephisto::tokenize(input);
        let mut ast = Mephisto::parse(tokens);

        let import_paths: Vec<_> = ast.imports();

        for path in import_paths {
            self.process_module(&path, context)?;
        }

        let symbol_table = Mephisto::create_symbol_table(&mut ast)?;

        if ast.errors.len() > 0 {
            module.errors = ast.errors.to_owned();
        }

        module.ast = ast;
        module.symbol_table = symbol_table;

        context.modules.insert(path.to_string(), module);

        Ok(())
    }

    fn load_module(&self, path: &str) -> Result<String, Box<dyn Error>> {
        let result: Result<String, Box<dyn Error>> = self.loader.load(path);
        result
    }
}
