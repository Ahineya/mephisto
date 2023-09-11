use std::error::Error;
use std::path::Path;
use indexmap::IndexMap;
use crate::codegen::{CodeGenerator};
use crate::ir::{IR, IRResult};

use crate::lexer::{Lexer, token::Token};
use crate::module_data::ModuleData;
use crate::module_loader::{FileLoader, StubFileLoader};
use crate::parser::ast::{AST};
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::symbol_table::SymbolTable;

use colored::Colorize;

pub mod lexer;
pub mod parser;
pub mod symbol_table;
pub mod semantic;

pub mod module_data;

pub mod module_loader;

pub mod ir;
pub mod codegen;

pub struct Mephisto<FL: FileLoader> {
    loader: FL,
}

#[derive(Debug)]
struct Context {
    loaded_modules: Box<Vec<String>>,
    modules: Box<IndexMap<String, ModuleData>>,
}

impl Mephisto<StubFileLoader> {
    pub fn tokenize(input: String) -> Vec<Token> {
        let lexer = Lexer::new();
        lexer.tokenize(input)
    }

    pub fn parse(tokens: Vec<Token>) -> AST {
        let mut parser = Parser::new();
        parser.parse(tokens)
    }

    pub fn create_symbol_table(ast: &mut AST) -> Result<SymbolTable, Vec<String>> {
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

    pub fn compile(&mut self, main_module_path: &str, codegen: Box<dyn CodeGenerator>) -> Result<String, Vec<String>> {
        let modules: IndexMap<String, ModuleData> = IndexMap::new();

        let mut context = Context {
            loaded_modules: Box::new(Vec::new()),
            modules: Box::new(modules),
        };

        let p: &Path = Path::new(main_module_path);

        // We want just to take the directory of the main module
        let current_dir = p.parent().unwrap_or(Path::new("."));
        let main_module_path = p.file_name().unwrap().to_str().unwrap();

        self.process_module(main_module_path, &mut context, Some(current_dir), p)?; // Recursively process all modules

        // println!("Modules: {:#?}", context);

        // For each module, check for errors
        let mut errors = Vec::new();
        for (path, module) in context.modules.iter() {
            if module.errors.len() > 0 {
                errors.extend(module.errors.iter().map(|e| format!("{}: {}", path, e)));
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        let mut modules = context.modules;

        let main_module = modules.get(main_module_path);

        if main_module.is_none() {
            return Err(vec![format!("Main module {} not found", main_module_path)]);
        }

        println!("{}", "Validating semantics...".blue());

        self.validate_semantics(&mut modules)?;

        for (path, module) in modules.iter() {
            if module.errors.len() > 0 {
                errors.extend(module.errors.iter().map(|e| format!("{}: {}", path, e)));
            }
        }

        let main_module = modules.get_mut(main_module_path).unwrap();

        let mut code = main_module.ast.to_code_string();

        code.push_str("\n\n");

        println!("{}", "Creating IR...".blue());

        // println!("Code: {}", code);
        let mut ir = IR::new();
        let ir_result = ir.create(&mut modules, main_module_path.to_string())?;

        // println!("IR: {:#?}", ir_result);

        // println!("Code: {}", ir_result.ast.to_code_string());

        if ir_result.errors.len() > 0 {
            errors.extend(ir_result.errors.iter().map(|e| format!("{}: {}", main_module_path, e)));
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        println!("{}", "Generating code...".blue());

        let code = self.generate_code(ir_result, codegen);

        code
    }

    fn process_module(&mut self, path: &str, context: &mut Context, base_path: Option<&Path>, current_path: &Path) -> Result<(), Vec<String>> {
        if context.loaded_modules.contains(&path.to_string()) {
            return Ok(());
        }

        let mut module = ModuleData::new();

        let input = self.load_module(path, base_path, current_path);

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

        let current_dir = base_path.unwrap_or(Path::new("."));
        // We want to join base path with the current path
        let cp = current_dir.join(path);
        let current_dir = cp.parent().unwrap_or(Path::new("."));

        for path in import_paths {
            self.process_module(&path, context, Some(&current_dir), current_path)?;
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

    fn load_module(&self, path: &str, base_path: Option<&Path>, current_path: &Path) -> Result<String, Box<dyn Error>> {
        let result: Result<String, Box<dyn Error>> = self.loader.load(path, base_path, current_path);
        result
    }

    pub fn generate_code(&self, ir: IRResult, code_generator: Box<dyn CodeGenerator>) -> Result<String, Vec<String>> {
        code_generator.generate(ir)
    }
}
