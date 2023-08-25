use std::collections::HashMap;

use indexmap::IndexMap;
use crate::lexer::token::Position;

use crate::module_data::ModuleData;
use crate::parser::ast::{AST, ASTTraverseStage, Node, traverse_ast};
use crate::symbol_table::{SymbolInfo, SymbolTable};

pub struct IR {
    pub errors: Vec<String>,
}

#[derive(Debug)]
pub struct IRResult {
    pub ast: AST,
    pub symbol_table: SymbolTable,
    pub errors: Vec<String>,
}

struct HoistingContext {
    name_counts: HashMap<String, usize>,
    symbol_table: SymbolTable,
    process_scope_index: Option<usize>,

    rename_symbols: bool,
}

impl HoistingContext {
    fn get_unique_name(&mut self, base_name: &str) -> String {
        let count = self.name_counts.entry(base_name.to_string()).or_insert(0);
        *count += 1;
        if *count == 1 {
            base_name.to_string()
        } else {
            format!("{}_{}", base_name, count)
        }
    }
}

impl IR {
    pub fn new() -> IR {
        IR {
            errors: vec![],
        }
    }

    // TODO: Rewrite so modules are not mutated
    pub fn create(&mut self, modules: &mut IndexMap<String, ModuleData>, main_module: String) -> Result<IRResult, Vec<String>> {
        // First pass should go through all modules and hoist all declarations from block and process nodes
        // Hoisting means that all declarations are moved to the top of the module and initialized with 0
        // Second pass should merge all modules into one
        // Third pass should inline all functions

        modules.iter_mut().for_each(|(_, module)| {
            let mut context = HoistingContext {
                name_counts: HashMap::new(),
                symbol_table: SymbolTable::new(),
                process_scope_index: None,

                rename_symbols: false,
            };

            context.process_scope_index = find_process_scope(&module.ast.root, &mut module.symbol_table);
            if context.process_scope_index.is_none() {
                return;
            }

            context.symbol_table = module.symbol_table.clone(); // TODO: Clone is not needed

            let global_symbols = module.symbol_table.get_global_symbol_names();

            for symbol_name in global_symbols {
                context.name_counts.insert(symbol_name.clone(), 1);
            }

            /*
                First pass: collect the symbols that should be hoisted
                Second pass: uniquely rename all symbols that should be hoisted both in the symbol table and in the AST
                Third pass: hoist the symbols both in the symbol table and in the AST
             */

            let symbols_to_hoist = collect_symbols_for_hoisting(&mut module.ast.root, &mut context);

            traverse_ast(&mut module.ast.root, &mut |stage, node, context: &mut HoistingContext| {
                match node {
                    Node::ProcessNode { .. } => {
                        match stage {
                            ASTTraverseStage::Enter => {
                                context.rename_symbols = true;
                            }
                            ASTTraverseStage::Exit => {
                                context.rename_symbols = false;
                            }
                        }
                    }

                    Node::Identifier { name, .. } => {
                        match stage {
                            ASTTraverseStage::Enter => {
                                if !context.rename_symbols {
                                    return false;
                                }

                                let process_scope_index = context.process_scope_index.unwrap();
                                let symbol = context.symbol_table.lookup_in_scope(name, process_scope_index);

                                if symbol.is_some() {
                                    let symbol = symbol.unwrap();

                                    // Check if the symbol should be hoisted
                                    symbols_to_hoist.iter().for_each(|(new_name, si)| {
                                        if si.id() == symbol.id() {
                                            // context.symbol_table.rename_symbol(symbol.id().clone(), new_name.clone());
                                            *name = new_name.clone();
                                        }
                                    });
                                }
                            }
                            _ => ()
                        }
                    }
                    _ => ()
                }

                false
            }, &mut context);

            // Rename the symbols in the symbol table

            let hoisted_nodes = hoist_process_block(&mut module.ast.root, &mut context);

            symbols_to_hoist.iter().for_each(|(new_name, si)| {
                context.symbol_table.rename_symbol(si.id().clone(), new_name.clone());
            });

            module.symbol_table = context.symbol_table;
            module.symbol_table.move_variables_to_global_scope(context.process_scope_index.unwrap());

            module.ast.root = Node::ProgramNode {
                children: hoisted_nodes,
                position: Position::new(),
            };

            // println!("RENAMED AST: {:#?}", module.ast);

            // todo!("Hoist the symbols in the AST and in the symbol table");

            // module.symbol_table.move_variables_to_global_scope(context.process_scope_index.unwrap());
        });

        let main_module_data = modules.get_mut(&main_module).unwrap();

        Ok(IRResult {
            ast: main_module_data.ast.clone(),
            symbol_table: main_module_data.symbol_table.clone(),
            errors: vec![],
        })
    }
}

fn find_process_scope(ast: &Node, symbol_table: &mut SymbolTable) -> Option<usize> {
    symbol_table.reset_scopes_indexes();

    match ast {
        Node::ProgramNode { children, .. } => {
            for node in children {
                match node {
                    Node::FunctionDeclarationStmt { .. } => {
                        symbol_table.enter_next_scope();
                        symbol_table.exit_scope();
                    }

                    Node::BlockNode { .. } => {
                        symbol_table.enter_next_scope();
                        symbol_table.exit_scope();
                    }

                    Node::BufferInitializer {
                        ..
                    } => {
                        symbol_table.enter_next_scope();
                        symbol_table.exit_scope();
                    }

                    Node::ProcessNode { .. } => {
                        symbol_table.enter_next_scope();
                        return Some(symbol_table.current_scope_index());
                    }
                    _ => (),
                }
            }

            None
        }
        _ => None,
    }
}

fn collect_symbols_for_hoisting(ast: &mut Node, context: &mut HoistingContext) -> Vec<(String, SymbolInfo)> {
    match ast {
        Node::ProgramNode { children, .. } => {
            let mut symbols_to_hoist: Vec<(String, SymbolInfo)> = Vec::new();

            for node in children {
                match node {
                    Node::ProcessNode { .. } => {
                        symbols_to_hoist.append(&mut collect_symbols_for_hoisting(node, context));
                    }
                    _ => ()
                }
            }

            symbols_to_hoist
        }

        Node::ProcessNode { children, .. } => {
            let mut symbols_to_hoist: Vec<(String, SymbolInfo)> = Vec::new();

            for node in children {
                match node {
                    Node::VariableDeclarationStmt { id,  .. } => {
                        let name = match id.as_ref() {
                            Node::Identifier { name, .. } => name,
                            _ => panic!("Expected identifier"),
                        };

                        let process_scope_index = context.process_scope_index.unwrap();
                        let symbol = context.symbol_table.lookup_in_scope(name, process_scope_index).unwrap();
                        symbols_to_hoist.push((context.get_unique_name(name), symbol.clone()));
                    }
                    _ => ()
                }
            }

            symbols_to_hoist
        }
        _ => vec![],
    }
}

fn hoist_process_block(ast: &mut Node, context: &mut HoistingContext) -> Vec<Node> {
    match ast {
        Node::ProgramNode { children, .. } => {
            let mut new_nodes = Vec::new();

            for node in children {
                match node {
                    Node::ProcessNode { .. } => {
                        new_nodes.append(&mut hoist_process_block(node, context));
                    }
                    _ => new_nodes.push(node.clone()),
                }
            }

            new_nodes
        }

        Node::ProcessNode { children, .. } => {
            let mut hoisted_declarations = Vec::new();
            let mut new_nodes = Vec::new();

            for node in children {
                match node {
                    Node::VariableDeclarationStmt { id, specifier, initializer, .. } => {
                        // Hoist the declaration with a default value
                        hoisted_declarations.push(Node::VariableDeclarationStmt {
                            id: id.clone(),
                            specifier: specifier.clone(),
                            initializer: Box::new(Node::Number {
                                value: 0.0,
                                position: Position::new(),
                            }),
                            position: Position::new(),
                        });

                        new_nodes.push(Node::AssignmentExpr {
                            lhs: id.clone(),
                            rhs: initializer.clone(),
                            position: Position::new(),
                        });
                    }
                    _ => new_nodes.push(node.clone()),
                }
            }

            // Return the hoisted declarations followed by the transformed ProcessNode
            let mut result = hoisted_declarations;
            result.push(Node::ProcessNode {
                children: new_nodes,
                position: ast.position().clone(),
            });

            result
        }
        _ => vec![ast.clone()],
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    use super::*;

    #[test]
    fn test_ir_lotion() {
        let code = "
            let foo = 42;

            bar(a, b) {
                return a + b;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let mut ir = IR::new();
        let result = ir.create(&mut modules, "main".to_string());

        assert!(result.is_ok());
    }

    #[test]
    fn test_ir_hoisting() {
        let code = "
            let foo = 42;

            process {
                let bar = 42;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let mut ir = IR::new();
        let result = ir.create(&mut modules, "main".to_string());

        assert!(result.is_ok());

        let mut ir_result = result.unwrap();

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("foo").is_some());
        assert!(ir_result.symbol_table.lookup("bar").is_some());
    }

    #[test]
    fn test_ir_hoisting_rename() {
        let code = "
            let foo = 42;

            process {
                let foo = 42;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let mut ir = IR::new();
        let result = ir.create(&mut modules, "main".to_string());

        assert!(result.is_ok());

        let mut ir_result = result.unwrap();

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("foo").is_some());
        assert!(ir_result.symbol_table.lookup("foo_2").is_some());
    }

    #[test]
    fn test_ir_hoisting_rename_2() {
        let code = "
            let foo = 42;

            process {
                let foo = 11;
                foo = 1;
                foo = 5;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let mut ir = IR::new();
        let result = ir.create(&mut modules, "main".to_string());

        assert!(result.is_ok());

        let mut ir_result = result.unwrap();

        ir_result.symbol_table.reset_scopes_indexes();

        println!("Code: {}", ir_result.ast.to_code_string());
        println!("Symbol table: {:#?}", ir_result.symbol_table);

        assert!(ir_result.symbol_table.lookup("foo").is_some());
        assert!(ir_result.symbol_table.lookup("foo_2").is_some());
    }
}
