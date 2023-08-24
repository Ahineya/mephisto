use std::collections::HashMap;

use indexmap::IndexMap;

use crate::lexer::token::Position;
use crate::module_data::ModuleData;
use crate::parser::ast::{AST, ASTTraverseStage, Node, traverse_ast, traverse_mut_ast};
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

struct HoistingContext {
    name_counts: HashMap<String, usize>,
    symbol_table: SymbolTable,
    process_scope_index: Option<usize>,
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
            let mut context = HoistingContext {
                name_counts: HashMap::new(),
                symbol_table: SymbolTable::new(),
                process_scope_index: None,
            };

            context.process_scope_index = find_process_scope(&module.ast.root, &mut module.symbol_table);
            context.symbol_table = module.symbol_table.clone(); // TODO: Clone is not needed

            let global_symbols = module.symbol_table.get_global_symbol_names();

            for symbol_name in global_symbols {
                context.name_counts.insert(symbol_name.clone(), 1);
            }

            module.ast.root = Node::ProgramNode {
                children: hoist_process_block(&module.ast.root, &mut context),
                position: Position::new(),
            };
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

fn hoist_process_block(mut ast: &Node, context: &mut HoistingContext) -> Vec<Node> {
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

            for mut node in children {
                match node {
                    Node::VariableDeclarationStmt { id, specifier, initializer, .. } => {
                        let mut id = id.clone();
                        let id = id.as_mut();

                        let name = match id { // looks like BS
                            Node::Identifier { name, .. } => name,
                            _ => panic!("Expected identifier"),
                        };

                        let unique_name = context.get_unique_name(name);
                        if unique_name != *name {
                            let name_symbol = context.symbol_table.lookup_in_scope(name, context.process_scope_index.unwrap()).unwrap();
                            let symbol_id = name_symbol.id();

                            let symbol_table_clone = context.symbol_table.clone();

                            let mut node = node.clone();

                            let mut stub_context = ();

                            traverse_ast(&mut node, &mut |stage, mut node, stub_context: &mut ()| {
                                match stage {
                                    ASTTraverseStage::Enter => {
                                        match node {
                                            Node::Identifier { name, .. } => {
                                                println!("Looking up {} in scope {}", name, context.process_scope_index.unwrap());
                                                let symbol = symbol_table_clone.lookup_in_scope(&name, context.process_scope_index.unwrap()).unwrap();
                                                if symbol.id() == symbol_id {
                                                    *name = unique_name.clone();
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }

                                false
                            }, &mut stub_context);

                            context.symbol_table.rename_variable(name, &unique_name, context.process_scope_index.unwrap());
                            context.symbol_table.move_variable_to_global_scope(name, context.process_scope_index.unwrap());
                        }

                        // Hoist the declaration with a default value
                        hoisted_declarations.push(Node::VariableDeclarationStmt {
                            id: Box::new(id.clone()),
                            specifier: specifier.clone(),
                            initializer: Box::new(Node::Number {
                                value: 0.0,
                                position: Position::new(),
                            }),
                            position: Position::new(),
                        });

                        new_nodes.push(Node::AssignmentExpr {
                            lhs: Box::new(id.clone()),
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

        println!("Symbol table: {:#?}", symbol_table);

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
