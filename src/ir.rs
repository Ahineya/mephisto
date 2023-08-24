use std::collections::HashMap;
use indexmap::IndexMap;

use crate::lexer::token::Position;
use crate::module_data::ModuleData;
use crate::parser::ast::{AST, Node};
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

    pub fn create(&mut self, modules: &mut IndexMap<String, ModuleData>, main_module: String) -> Result<IRResult, Vec<String>> {
        // First pass should go through all modules and hoist all declarations from block and process nodes
        // Hoisting means that all declarations are moved to the top of the module and initialized with 0
        // Second pass should merge all modules into one
        // Third pass should inline all functions

        modules.iter_mut().for_each(|(_, module)| {
            module.ast.root = Node::ProgramNode {
                children: hoist_process_block(&module.ast.root),
                position: Position::new(),
            };

            let process_scope_index = find_process_scope(&module.ast.root, &mut module.symbol_table);

            if process_scope_index.is_none() {
                return;
            }

            let process_scope_index = process_scope_index.unwrap();
            module.symbol_table.move_variables_to_global_scope(process_scope_index);
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

fn hoist_process_block(ast: &Node) -> Vec<Node> {
    match ast {
        Node::ProgramNode { children, .. } => {
            let mut new_nodes = Vec::new();

            for node in children {
                match node {
                    Node::ProcessNode { .. } => {
                        new_nodes.append(&mut hoist_process_block(node));
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

            // Return the hoisted declarations followed by the transformed ProcessBlock
            let mut result = hoisted_declarations;
            result.push(Node::ProcessNode {
                children: new_nodes,
                position: ast.position().clone(),
            });

            println!("Result: {:#?}", result);

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
    #[ignore]
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

        println!("Symbol table: {:#?}", ir_result.symbol_table);
        println!("AST: {:#?}", ir_result.ast);

        assert!(ir_result.symbol_table.lookup("foo").is_some());
        assert!(ir_result.symbol_table.lookup("foo_1").is_some());
    }
}
