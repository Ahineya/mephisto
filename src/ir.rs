use std::collections::{HashMap, HashSet};

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
            format!("#{}_{}", base_name, count)
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

        Self::hoist(modules);

        // TODO: Merge all modules into one

        /*
          How to merge modules:
            1. Create a new module with an empty AST and symbol table
            2. When the module is imported, it is "instantiated". Meaning that importing the same module twice will create two instances of the module
            3. When a module is instantiated, all its symbols are renamed to be unique
         */

        // So, the main module is the one that is being executed. Then we need to recursively go through all the modules imported by the main module and merge them into the main module
        // The main module should be the one that is being executed, so it should be the one that contains the process node

        let mut processed_modules = HashSet::new();
        let merged_module = Self::merge_modules(modules, &main_module, &mut processed_modules);
        let mut with_replaced_module_calls = Self::replace_module_calls(&merged_module.ast.root, &merged_module.symbol_table);
        let with_replaced_stdlib_calls = Self::replace_stdlib_calls(&with_replaced_module_calls.ast.root, &mut with_replaced_module_calls.symbol_table);

        let main_module_data = with_replaced_stdlib_calls;

        Ok(IRResult {
            ast: main_module_data.ast.clone(),
            symbol_table: main_module_data.symbol_table.clone(),
            errors: vec![],
        })
    }

    fn replace_stdlib_calls(ast: &Node, symbol_table: &mut SymbolTable) -> ModuleData {
        let mut result = ast.clone();
        let mut context = ();

        let symbols_to_rename = Self::collect_stdlib_symbols(&result, symbol_table);
        symbol_table.reset_scopes_indexes();

        traverse_ast(&mut result, &mut |stage, node, context: &mut ()| {
            match node {
                Node::BlockNode { .. }
                |
                Node::BufferInitializer { .. }
                |
                Node::FunctionBody { .. }
                |
                Node::ProcessNode { .. }
                => {
                    match stage {
                        ASTTraverseStage::Enter => {
                            symbol_table.enter_next_scope();
                        }
                        ASTTraverseStage::Exit => {
                            symbol_table.exit_scope();
                        }
                    }
                }

                Node::Identifier { name, .. } => {
                    match stage {
                        ASTTraverseStage::Enter => {
                            let symbol = symbol_table.lookup(name);

                            if symbol.is_some() {
                                let symbol = symbol.unwrap();

                                symbols_to_rename.iter().for_each(|(new_name, si)| {
                                    if si.id() == symbol.id() {
                                        *name = format!("##STD_{}", new_name);
                                    }
                                });
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            false
        }, &mut context);

        ModuleData {
            ast: AST::new(result.clone(), vec![]),
            symbol_table: SymbolTable::from_ast(&mut AST::new(result.clone(), vec![])).unwrap(), // TODO: This is cheating, make it better
            errors: vec![],
        }
    }

    fn collect_stdlib_symbols(ast: &Node, symbol_table: &SymbolTable) -> Vec<(String, SymbolInfo)> {
        symbol_table.get_stdlib_symbols()
    }

    fn replace_module_calls(ast: &Node, symbol_table: &SymbolTable) -> ModuleData {
        let mut result = ast.clone();
        let mut context = ();

        traverse_ast(&mut result, &mut |stage, node, context: &mut ()| {
            match node {
                Node::MemberExpr { object, property, .. } => {
                    match stage {
                        ASTTraverseStage::Enter => {
                            let object_name = match object.as_ref() {
                                Node::Identifier { name, .. } => name,
                                _ => panic!("Expected identifier"),
                            };

                            let property_name = match property.as_ref() {
                                Node::Identifier { name, .. } => name,
                                _ => panic!("Expected identifier"),
                            };

                            // Replace the module call with the module's symbol
                            *node = Node::Identifier {
                                name: format!("{}#{}", object_name, property_name),
                                position: Position::new(),
                            };
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            false
        }, &mut context);

        ModuleData {
            ast: AST::new(result.clone(), vec![]),
            symbol_table: SymbolTable::from_ast(&mut AST::new(result.clone(), vec![])).unwrap(), // TODO: This is cheating, make it better
            errors: vec![],
        }
    }

    fn merge_modules(modules: &mut IndexMap<String, ModuleData>, module_name: &str, processed_modules: &mut HashSet<String>) -> ModuleData {
        let mut result = ModuleData::new();
        let mut imported_process_node: Option<Node> = None;
        let mut imported_block_node: Option<Node> = None;
        let mut imported_connect_node: Option<Node> = None;

        // If the module has already been processed, just return it
        if processed_modules.contains(module_name) {
            return modules.get(module_name).expect("Processed module not found").clone();
        }

        let module = modules.remove(module_name).expect("Module not found");
        result.symbol_table = module.symbol_table.clone();

        if let Node::ProgramNode { children, .. } = &module.ast.root {
            if let Node::ProgramNode { children: result_children, .. } = &mut result.ast.root {
                for node in children.iter() {
                    match node {
                        Node::ImportStatement { id, path, .. } => {
                            // Recursively merge the imported module
                            let mut imported_module = Self::merge_modules(modules, &path, processed_modules);

                            let id = match id.as_ref() {
                                Node::Identifier { name, .. } => name,
                                _ => panic!("Expected identifier"),
                            };

                            let renamed_node = Self::rename_symbols(&imported_module.ast.root, id, &mut imported_module.symbol_table);

                            if let Node::ProgramNode { children: renamed_children, .. } = &renamed_node {

                                for renamed_node in renamed_children.iter() {

                                    match renamed_node {
                                        Node::ProcessNode { .. } => {
                                            imported_process_node = Some(renamed_node.clone());
                                        }
                                        Node::BlockNode { .. } => {
                                            imported_block_node = Some(renamed_node.clone());
                                        }
                                        Node::ConnectNode { .. } => {
                                            imported_connect_node = Some(renamed_node.clone());
                                        }
                                        _ => {
                                            result_children.push(renamed_node.clone());
                                        }
                                    }
                                }
                            }
                        }

                        _ => {
                            result_children.push(node.clone());
                        }
                    }
                }

                // After iterating through all children, handle the imported nodes
                if let Some(imported_node) = imported_block_node {
                    if let Some(Node::BlockNode { children: main_block_children, .. }) = result_children.iter_mut().find(|n| matches!(n, Node::BlockNode { .. })) {
                        if let Node::BlockNode { children: imported_block_children, .. } = &imported_node {
                            main_block_children.splice(0..0, imported_block_children.clone());
                        }
                    } else {
                        result_children.push(imported_node);
                    }
                }

                if let Some(imported_node) = imported_process_node {
                    if let Some(Node::ProcessNode { children: main_process_children, .. }) = result_children.iter_mut().find(|n| matches!(n, Node::ProcessNode { .. })) {
                        if let Node::ProcessNode { children: imported_process_children, .. } = &imported_node {
                            main_process_children.splice(0..0, imported_process_children.clone());
                        }
                    } else {
                        result_children.push(imported_node);
                    }
                }

                if let Some(imported_node) = imported_connect_node {
                    if let Some(Node::ConnectNode { children: main_connect_children, .. }) = result_children.iter_mut().find(|n| matches!(n, Node::ConnectNode { .. })) {
                        if let Node::ConnectNode { children: imported_connect_children, .. } = &imported_node {
                            main_connect_children.splice(0..0, imported_connect_children.clone());
                        }
                    } else {
                        result_children.push(imported_node);
                    }
                }
            }
        }

        processed_modules.insert(module_name.to_string());
        // Re-insert the merged module into the modules map
        modules.insert(module_name.to_string(), result.clone());

        result.symbol_table = SymbolTable::from_ast(&mut result.ast).unwrap(); // TODO: This is cheating, make it better

        result
    }


    fn rename_symbols(node: &Node, module_id: &str, symbol_table: &mut SymbolTable) -> Node {
        let mut renamed_node = node.clone();
        let mut context = HoistingContext {
            name_counts: HashMap::new(),
            symbol_table: symbol_table.clone(),
            process_scope_index: None,

            rename_symbols: false,
        };

        let symbols_to_rename = collect_symbols_for_rename(&mut renamed_node, &mut context);

        context.symbol_table.reset_scopes_indexes();

        traverse_ast(&mut renamed_node, &mut |stage, node, context: &mut HoistingContext| {
            match node {
                Node::BlockNode { .. }
                |
                Node::BufferInitializer { .. }
                |
                Node::FunctionBody { .. }
                |
                Node::ProcessNode { .. }
                => {
                    match stage {
                        ASTTraverseStage::Enter => {
                            context.symbol_table.enter_next_scope();
                        }
                        ASTTraverseStage::Exit => {
                            context.symbol_table.exit_scope();
                        }
                    }
                }

                Node::Identifier { name, .. } => {
                    match stage {
                        ASTTraverseStage::Enter => {
                            let symbol = context.symbol_table.lookup(name);

                            if symbol.is_some() {
                                let symbol = symbol.unwrap();

                                // Check if the symbol should be renamed
                                symbols_to_rename.iter().for_each(|(new_name, si)| {
                                    if si.id() == symbol.id() {
                                        // new name is the name of the module + the new name
                                        *name = format!("{}#{}", module_id, new_name);
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

        renamed_node
    }

    fn hoist(modules: &mut IndexMap<String, ModuleData>) {
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

            context.symbol_table = module.symbol_table.clone(); // TODO: Clone is not needed, make it better

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
        });
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
                    Node::VariableDeclarationStmt { id, .. } => {
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

fn collect_symbols_for_rename(ast: &mut Node, context: &mut HoistingContext) -> Vec<(String, SymbolInfo)> {
    let mut symbols_to_rename: Vec<(String, SymbolInfo)> = Vec::new();

    context.symbol_table.reset_scopes_indexes();

    // We want to collect all variable declarations in all scopes here
    traverse_ast(ast, &mut |stage, node, context: &mut HoistingContext| {
        match node {
            Node::BlockNode { .. }
            |
            Node::BufferInitializer { .. }
            |
            Node::FunctionBody { .. }
            |
            Node::ProcessNode { .. }
            => {
                match stage {
                    ASTTraverseStage::Enter => {
                        context.symbol_table.enter_next_scope();
                    }
                    ASTTraverseStage::Exit => {
                        context.symbol_table.exit_scope();
                    }
                }
            }

            Node::VariableDeclarationStmt { id, .. }
            |
            Node::FunctionDeclarationStmt { id, .. }
            |
            Node::BufferDeclarationStmt {id, ..}
            |
            Node::ParameterDeclarationStmt {id, ..} => {
                match stage {
                    ASTTraverseStage::Enter => {
                        let name = match id.as_ref() {
                            Node::Identifier { name, .. } => name,
                            _ => panic!("Expected identifier"),
                        };

                        let symbol = context.symbol_table.lookup_in_scope(name, context.symbol_table.current_scope_index());

                        if symbol.is_some() {
                            let symbol = symbol.unwrap();
                            symbols_to_rename.push((context.get_unique_name(name), symbol.clone()));
                        } else {
                            println!("Symbol {} not found", name.to_string());
                        }
                    }
                    _ => ()
                }
            }

            _ => ()
        }

        false
    }, context);

    symbols_to_rename
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

        println!("AST: {:#?}", ir_result.ast);

        assert!(ir_result.symbol_table.lookup("foo").is_some());
        assert!(ir_result.symbol_table.lookup("#foo_2").is_some());
    }

    #[test]
    fn test_ir_hoisting_rename_2() {
        let code = "
            let foo = 42;

            process {
                let foo = 11;
                foo = 1;
                foo = 5;

                spoo(foo1) {
                    let a = foo1 + 1;
                    return a + foo;
                }
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
        assert!(ir_result.symbol_table.lookup("#foo_2").is_some());
    }

    #[test]
    fn test_ir_hoisting_rename_builtins() {
        let code = "
            let foo = 42;

            process {
                let sin = 11;
                sin = 1;
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
        assert!(ir_result.symbol_table.lookup("#sin_2").is_some());
    }

    #[test]
    fn test_multiple_modules() {
        let main_code = "
            import Mod from \"./module.meph\";

            output a = 0;

            process {
                a = Mod.add(Mod.out, Mod.PI);
            }

            connect {
                a -> OUTPUTS;
            }

            ".to_string();

        let module_code = "
            param something {
               initial: 42;
            };

            output out = 0;
            export const PI = 3.14;

            export add(a, b) {
                return a + b + something;
            }

            process {
                out = 42;
            }
        ".to_string();

        let lexer = Lexer::new();
        let main_tokens = lexer.tokenize(main_code);
        let module_tokens = lexer.tokenize(module_code);

        let mut parser = Parser::new();
        let mut main_ast = parser.parse(main_tokens);
        let mut module_ast = parser.parse(module_tokens);

        let main_symbol_table = SymbolTable::from_ast(&mut main_ast).unwrap();
        let module_symbol_table = SymbolTable::from_ast(&mut module_ast).unwrap();

        let main_data = ModuleData {
            ast: main_ast,
            symbol_table: main_symbol_table,
            errors: vec![],
        };

        let module_data = ModuleData {
            ast: module_ast,
            symbol_table: module_symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), main_data);
        modules.insert("./module.meph".to_string(), module_data);

        let mut ir = IR::new();
        let result = ir.create(&mut modules, "main".to_string());

        assert!(result.is_ok());

        let mut ir_result = result.unwrap();

        println!("AST: {:#?}", ir_result.ast);
        println!("{}", ir_result.ast.to_code_string());

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("Mod#PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod#add").is_some());
    }

    #[test]
    fn test_multiple_modules_multiple_same_module_import() {
        let main_code = "
            import Mod from \"./module.meph\";
            import Mod2 from \"./module.meph\";

            output a = 0;

            process {
                a = Mod.add(Mod.out, Mod.PI) + Mod2.add(Mod2.out, Mod2.PI);
            }

            connect {
                a -> OUTPUTS;
            }

            ".to_string();

        let module_code = "
            output out = 0;
            export const PI = 3.14;

            export add(a, b) {
                return a + b + something;
            }

            process {
                out = 42;
            }
        ".to_string();

        let lexer = Lexer::new();
        let main_tokens = lexer.tokenize(main_code);
        let module_tokens = lexer.tokenize(module_code);

        let mut parser = Parser::new();
        let mut main_ast = parser.parse(main_tokens);
        let mut module_ast = parser.parse(module_tokens);

        let main_symbol_table = SymbolTable::from_ast(&mut main_ast).unwrap();
        let module_symbol_table = SymbolTable::from_ast(&mut module_ast).unwrap();

        let main_data = ModuleData {
            ast: main_ast,
            symbol_table: main_symbol_table,
            errors: vec![],
        };

        let module_data = ModuleData {
            ast: module_ast,
            symbol_table: module_symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), main_data);
        modules.insert("./module.meph".to_string(), module_data);

        let mut ir = IR::new();
        let result = ir.create(&mut modules, "main".to_string());

        assert!(result.is_ok());

        let mut ir_result = result.unwrap();

        println!("AST: {:#?}", ir_result.ast);
        println!("{}", ir_result.ast.to_code_string());

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("Mod#PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod#add").is_some());

        assert!(ir_result.symbol_table.lookup("Mod2#PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod2#add").is_some());
    }

    #[test]
    fn test_multiple_modules_multiple_same_module_import_with_param() {
        let main_code = "
            import Mod from \"./module.meph\";
            import Mod2 from \"./module.meph\";

            output a = 0;

            process {
                a = Mod.add(Mod.out, Mod.PI) + Mod2.add(Mod2.out, Mod2.PI);
            }

            connect {
                a -> OUTPUTS;
            }

            ".to_string();

        let module_code = "
            param something {
               initial: 42;
            };

            output out = 0;
            input in = 0;

            buffer b[10];

            buffer b2[10] = |i| {
                return i + 1;
            };

            export const PI = 3.14 + sin(25);

            export add(a, b) {
                return a + b + something;
            }

            process {
                out = 42;
            }

            connect {
                out -> in;
            }
        ".to_string();

        let lexer = Lexer::new();
        let main_tokens = lexer.tokenize(main_code);
        let module_tokens = lexer.tokenize(module_code);

        let mut parser = Parser::new();
        let mut main_ast = parser.parse(main_tokens);
        let mut module_ast = parser.parse(module_tokens);

        let main_symbol_table = SymbolTable::from_ast(&mut main_ast).unwrap();
        let module_symbol_table = SymbolTable::from_ast(&mut module_ast).unwrap();

        let main_data = ModuleData {
            ast: main_ast,
            symbol_table: main_symbol_table,
            errors: vec![],
        };

        let module_data = ModuleData {
            ast: module_ast,
            symbol_table: module_symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), main_data);
        modules.insert("./module.meph".to_string(), module_data);

        let mut ir = IR::new();
        let result = ir.create(&mut modules, "main".to_string());

        assert!(result.is_ok());

        let mut ir_result = result.unwrap();

        println!("{}", ir_result.ast.to_code_string());

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("Mod#PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod#add").is_some());

        assert!(ir_result.symbol_table.lookup("Mod2#PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod2#add").is_some());
    }

    #[test]
    fn test_multiple_modules_tree() {
        let main_code = "
            import Mod from \"./module.meph\";

            output a = 0;

            process {
                a = Mod.add(Mod.out, Mod.PI) + Mod2.add(Mod2.out, Mod2.PI);
            }

            connect {
                a -> OUTPUTS;
            }

            ".to_string();

        let module_code = "
            import Mod2 from \"./module2.meph\";

            param something {
               initial: 42;
            };

            output out = 0;
            input in = 0;

            buffer b[10];

            buffer b2[10] = |i| {
                return i + 1;
            };

            export const PI = 3.14 + sin(25);

            export add(a, b) {
                return a + b + something;
            }

            process {
                out = 42;
            }

            connect {
                out -> in;
            }
        ".to_string();

        let module2_code = "
            export const E = 2.71828;

            process {
                out = 42 + E;
            }
        ".to_string();

        let lexer = Lexer::new();
        let main_tokens = lexer.tokenize(main_code);
        let module_tokens = lexer.tokenize(module_code);

        let mut parser = Parser::new();
        let mut main_ast = parser.parse(main_tokens);
        let mut module_ast = parser.parse(module_tokens);
        let mut module2_ast = parser.parse(lexer.tokenize(module2_code));

        let main_symbol_table = SymbolTable::from_ast(&mut main_ast).unwrap();
        let module_symbol_table = SymbolTable::from_ast(&mut module_ast).unwrap();
        let module2_symbol_table = SymbolTable::from_ast(&mut module2_ast).unwrap();

        let main_data = ModuleData {
            ast: main_ast,
            symbol_table: main_symbol_table,
            errors: vec![],
        };

        let module_data = ModuleData {
            ast: module_ast,
            symbol_table: module_symbol_table,
            errors: vec![],
        };

        let module2_data = ModuleData {
            ast: module2_ast,
            symbol_table: module2_symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), main_data);
        modules.insert("./module.meph".to_string(), module_data);
        modules.insert("./module2.meph".to_string(), module2_data);

        let mut ir = IR::new();
        let result = ir.create(&mut modules, "main".to_string());

        assert!(result.is_ok());

        let mut ir_result = result.unwrap();

        println!("{}", ir_result.ast.to_code_string());

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("Mod#PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod#add").is_some());

        assert!(ir_result.symbol_table.lookup("Mod#Mod2#E").is_some());
    }
}
