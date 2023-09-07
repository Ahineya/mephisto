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
    pub input_names: Vec<String>,
    pub output_names: Vec<String>,
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
        // Third pass should inline all functions (skip this for now)
        // Fourth pass should rename all inputs, outputs, and params to array accesses

        Self::hoist(modules);

        let mut processed_modules = HashSet::new();

        let merged_module = Self::merge_modules(modules, &main_module, &mut processed_modules);
        let mut with_replaced_module_calls = Self::replace_module_calls(&merged_module.ast.root, &merged_module.symbol_table);
        let mut with_replaced_stdlib_calls = Self::replace_stdlib_calls(&with_replaced_module_calls.ast.root, &mut with_replaced_module_calls.symbol_table);
        let (with_replaced_connects, input_names, output_names) = Self::replace_connects(&with_replaced_stdlib_calls.ast, &mut with_replaced_stdlib_calls.symbol_table);

        let main_module_data = with_replaced_connects;

        Ok(IRResult {
            ast: main_module_data.ast.clone(),
            symbol_table: main_module_data.symbol_table.clone(),
            input_names,
            output_names,
            errors: vec![],
        })
    }

    fn replace_connects(ast: &AST, symbol_table: &mut SymbolTable) -> (ModuleData, Vec<String>, Vec<String>) {
        let mut result = ast.clone();
        symbol_table.reset_scopes_indexes();

        // 1st pass — collect all input and output symbols
        // 2nd pass — rename all input and output symbols

        let mut context = ();

        let mut input_symbols: Vec<(String, SymbolInfo)> = vec![];
        let mut output_symbols: Vec<(String, SymbolInfo)> = vec![];

        let skip_renaming_identifiers = false;
        let mut skip_renaming_once = false;

        let input_names = result.inputs();
        let output_names = result.outputs();

        for name in input_names {
            let symbol = symbol_table.lookup(&name);
            if symbol.is_some() {
                let symbol = symbol.unwrap();

                if symbol.is_input() {
                    input_symbols.push((name.clone(), symbol.clone()));
                }
            }
        }

        for name in output_names {
            let symbol = symbol_table.lookup(&name);
            if symbol.is_some() {
                let symbol = symbol.unwrap();

                if symbol.is_output() && !symbol.is_parameter() {
                    output_symbols.push((name.clone(), symbol.clone()));
                }
            }
        }

        symbol_table.reset_scopes_indexes();

        traverse_ast(&mut result.root, &mut |stage, node, _context: &mut ()| {
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

                Node::ParameterDeclarationField {
                    ..
                } => {
                    match stage {
                        ASTTraverseStage::Enter => {
                            skip_renaming_once = true;
                        }
                        ASTTraverseStage::Exit => {
                            skip_renaming_once = false;
                        }
                    }
                }

                Node::Identifier { name, .. } => {
                    if skip_renaming_identifiers {
                        return false;
                    }

                    if skip_renaming_once {
                        skip_renaming_once = false;
                        return false;
                    }
                    match stage {
                        ASTTraverseStage::Enter => {
                            let symbol = symbol_table.lookup(name);

                            if symbol.is_some() {
                                let symbol = symbol.unwrap();

                                input_symbols.iter().enumerate().for_each(|(i, (_, si))| {
                                    if si.id() == symbol.id() {
                                        *name = format!("##INPUT_[{}]", i);
                                    }
                                });

                                output_symbols.iter().enumerate().for_each(|(i, (_, si))| {
                                    if si.id() == symbol.id() {
                                        *name = format!("##OUTPUT_[{}]", i);
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

        let input_symbol_names: Vec<String> = input_symbols.iter().map(|(name, _)| name.clone()).collect();
        let output_symbol_names: Vec<String> = output_symbols.iter().map(|(name, _)| name.clone()).collect();

        (
            ModuleData {
                ast: AST::new(result.root.clone(), vec![]),
                symbol_table: SymbolTable::from_ast(&mut AST::new(result.root.clone(), vec![])).unwrap(), // TODO: This is cheating, make it better
                errors: vec![],
            },
            input_symbol_names,
            output_symbol_names,
        )
    }

    fn replace_stdlib_calls(ast: &Node, symbol_table: &mut SymbolTable) -> ModuleData {
        let mut result = ast.clone();
        let skip_renaming_identifiers = false;
        let mut skip_renaming_once = false;

        let symbols_to_rename = Self::collect_stdlib_symbols(&result, symbol_table);
        symbol_table.reset_scopes_indexes();

        let mut context = ();

        traverse_ast(&mut result, &mut |stage, node, _context: &mut ()| {
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

                Node::ParameterDeclarationField {
                    ..
                } => {
                    match stage {
                        ASTTraverseStage::Enter => {
                            skip_renaming_once = true;
                        }
                        ASTTraverseStage::Exit => {
                            skip_renaming_once = false;
                        }
                    }
                }

                Node::Identifier { name, .. } => {
                    if skip_renaming_identifiers {
                        return false;
                    }

                    if skip_renaming_once {
                        skip_renaming_once = false;
                        return false;
                    }

                    match stage {
                        ASTTraverseStage::Enter => {
                            let symbol = symbol_table.lookup(name);

                            // println!("Renaming symbol {}", name);

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

    fn collect_stdlib_symbols(_ast: &Node, symbol_table: &SymbolTable) -> Vec<(String, SymbolInfo)> {
        symbol_table.get_stdlib_symbols()
    }

    fn replace_module_calls(ast: &Node, _symbol_table: &SymbolTable) -> ModuleData {
        let mut result = ast.clone();
        let mut context = ();

        traverse_ast(&mut result, &mut |stage, node, _| {
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

        // let mut imported_process_node: Option<Node> = None;
        // let mut imported_block_node: Option<Node> = None;
        // let mut imported_connect_node: Option<Node> = None;

        let mut imported_process_nodes: Vec<Node> = Vec::new();
        let mut imported_block_nodes: Vec<Node> = Vec::new();
        let mut imported_connect_nodes: Vec<Node> = Vec::new();

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
                            let imported_module = Self::merge_modules(modules, &path, processed_modules);
                            let mut module = Self::replace_module_calls(&imported_module.ast.root, &imported_module.symbol_table);

                            // println!("{}", module.ast.to_code_string());

                            let id = match id.as_ref() {
                                Node::Identifier { name, .. } => name,
                                _ => panic!("Expected identifier"),
                            };

                            let renamed_node = Self::rename_symbols(&module.ast.root, id, &mut module.symbol_table);

                            if let Node::ProgramNode { children: renamed_children, .. } = &renamed_node {
                                for renamed_node in renamed_children.iter() {
                                    match renamed_node {
                                        Node::ProcessNode { .. } => {
                                            imported_process_nodes.push(renamed_node.clone());
                                        }
                                        Node::BlockNode { .. } => {
                                            imported_block_nodes.push(renamed_node.clone());
                                        }
                                        Node::ConnectNode { .. } => {
                                            imported_connect_nodes.push(renamed_node.clone());
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
                for imported_node in imported_block_nodes {
                    if let Some(Node::BlockNode { children: main_block_children, .. }) = result_children.iter_mut().find(|n| matches!(n, Node::BlockNode { .. })) {
                        if let Node::BlockNode { children: imported_block_children, .. } = &imported_node {
                            main_block_children.splice(0..0, imported_block_children.clone());
                        }
                    } else {
                        result_children.push(imported_node);
                    }
                }

                for imported_node in imported_process_nodes {
                    if let Some(Node::ProcessNode { children: main_process_children, .. }) = result_children.iter_mut().find(|n| matches!(n, Node::ProcessNode { .. })) {
                        if let Node::ProcessNode { children: imported_process_children, .. } = &imported_node {
                            main_process_children.splice(0..0, imported_process_children.clone());
                        }
                    } else {
                        result_children.push(imported_node);
                    }
                }

                for imported_node in imported_connect_nodes {
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

        // let mut module = Self::replace_module_calls(&result.ast.root, &result.symbol_table);
        // module.symbol_table = SymbolTable::from_ast(&mut module.ast).unwrap(); // TODO: This is cheating, make it better
        //
        // module
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

        // println!("Symbols to rename: {:#?}", symbols_to_rename);
        // println!("AST before renaming: {:#?}", renamed_node);

        context.symbol_table.reset_scopes_indexes();

        context.rename_symbols = true;

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

                Node::ParameterDeclarationField {
                    ..
                } => {
                    match stage {
                        ASTTraverseStage::Enter => {
                            context.rename_symbols = false;
                        }
                        ASTTraverseStage::Exit => {
                            context.rename_symbols = true;
                        }
                    }
                }

                Node::FunctionParameter {
                    ..
                } => {
                    match stage {
                        ASTTraverseStage::Enter => {
                            context.rename_symbols = false;
                        }
                        ASTTraverseStage::Exit => {
                            context.rename_symbols = true;
                        }
                    }
                }

                Node::Identifier { name, .. } => {
                    match stage {
                        ASTTraverseStage::Enter => {
                            if !context.rename_symbols {
                                return false;
                            }

                            // println!("Renaming symbol {}", name);

                            let symbol = context.symbol_table.lookup(name);

                            if symbol.is_none() {
                                println!("Symbol not found");
                                // println!("Symbol table: {:#?}", context.symbol_table);
                            }

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

            // println!("Symbol table before hoisting: {:#?}", context.symbol_table);

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

    let mut context: (&mut SymbolTable, Option<usize>) = (
        symbol_table,
        None
    );

    let mut ast = ast.clone();

    traverse_ast(&mut ast, &mut |stage, node, context: &mut (&mut SymbolTable, Option<usize>)| {
        match node {
            Node::BlockNode { .. }
            |
            Node::BufferInitializer { .. }
            |
            Node::FunctionBody { .. }
            => {
                match stage {
                    ASTTraverseStage::Enter => {
                        context.0.enter_next_scope();
                    }
                    ASTTraverseStage::Exit => {
                        context.0.exit_scope();
                    }
                }
            }

            Node::ProcessNode { .. } => {
                match stage {
                    ASTTraverseStage::Enter => {
                        context.0.enter_next_scope();
                        context.1 = Some(context.0.current_scope_index());
                    }
                    ASTTraverseStage::Exit => {
                        context.0.exit_scope();
                    }
                }
            }
            _ => ()
        }

        false
    }, &mut context);

    context.1
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
                    Node::ExpressionStmt { child, .. } => {
                        match child.as_ref() {
                            Node::VariableDeclarationStmt { id, .. } => {
                                let name = match id.as_ref() {
                                    Node::Identifier { name, .. } => name,
                                    _ => panic!("Expected identifier"),
                                };

                                let process_scope_index = context.process_scope_index.unwrap();

                                // println!("Looking up {} in scope {}", name, process_scope_index);
                                // println!("Symbol table: {:#?}", context.symbol_table);

                                let symbol = context.symbol_table.lookup_in_scope(name, process_scope_index).unwrap();
                                symbols_to_hoist.push((context.get_unique_name(name), symbol.clone()));
                            }
                            _ => ()
                        }
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
            Node::BufferDeclarationStmt { id, .. }
            |
            Node::ParameterDeclarationStmt { id, .. } => {
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
                    Node::ExpressionStmt { child, .. } => {
                        match child.as_ref() {
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

        // println!("code: {}", ir_result.ast.to_code_string());

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

        // println!("AST: {:#?}", ir_result.ast);

        assert!(ir_result.symbol_table.lookup("foo").is_some());
        assert!(ir_result.symbol_table.lookup("#foo_2").is_some());
    }

    #[test]
    fn test_hoisting_no_rename_shadowed() {
        let code = "
            process {
                const PI = 3.1415926535897932384626433832795028841971693993751058209749445923078164062;
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

        // println!("CODE: {}", ir_result.ast.to_code_string());

        assert!(ir_result.symbol_table.lookup("PI").is_some());
    }

    #[test]
    fn test_ir_hoisting_rename_2() {
        let code = "
            let foo = 42;

            process {
                let foo = 11;
                foo = 1;
                foo = 5;

                fn spoo(foo1) {
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
    fn test_ir_hoisting_full() {
        let code = "
param frequency {
    min: 40; // Shouldn't be renamed!!!
    max: 22000;
    step: 1;
    initial: 220;
};

let a = 1;

buffer b[1024];

buffer moo[10] = |i| {
    return i * 2;
};

output out = 0;

let phase = 0;
let increment = 0;

//const SR = 44100;

input gain = 1 + 0.5 * getSin(0.5);
input kick = 0;

block {
    increment = frequency / SR;
    return 123;
}

getSaw(phase) {
    return phase * 2 - 1;
}

//export const PI = 3.14;

export getSin(phase) {
    let b = 1;
    return sin(phase * 2 * PI);
}

process {
    const PI = 3.1415926535897932384626433832795028841971693993751058209749445923078164062;
    phase = increment + (phase - floor(increment + phase));
    out = (phase > -0.5) * 2 - 1;
    out = out * gain;

    let a = 0;

    const test = floor(2.5);

    getPoo() {
        return 1;
    }

    a = 123;

    //let a = 0;

    return a + 1.1;
}

connect {
    //out -> OUTPUTS[0];
    out -> OUTPUTS;

    //phase -> Kick.phase;
    //gain -> Kick.gain;

    //Kick.out -> kick;
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
            export const M_PI = 3.14;

            export fn add(a, b) {
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

        // println!("AST: {:#?}", ir_result.ast);
        // println!("{}", ir_result.ast.to_code_string());

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("Mod#M_PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod#add").is_some());
    }

    #[test]
    fn test_multiple_modules_multiple_same_module_import() {
        let main_code = "
            import Mod from \"./module.meph\";
            import Mod2 from \"./module.meph\";

            output a = 0;

            process {
                a = Mod.add(Mod.out, Mod.M_PI) + Mod2.add(Mod2.out, Mod2.M_PI);
            }

            connect {
                a -> OUTPUTS;
            }

            ".to_string();

        let module_code = "
            output out = 0;
            export const M_PI = 3.14;

            export fn add(a, b) {
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

        // println!("AST: {:#?}", ir_result.ast);
        // println!("{}", ir_result.ast.to_code_string());

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("Mod#M_PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod#add").is_some());

        assert!(ir_result.symbol_table.lookup("Mod2#M_PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod2#add").is_some());
    }

    #[test]
    fn test_multiple_modules_multiple_same_module_import_with_param() {
        let main_code = "
            import Mod from \"./module.meph\";
            import Mod2 from \"./module.meph\";

            output a = 0;

            process {
                a = Mod.add(Mod.out, Mod.M_PI) + Mod2.add(Mod2.out, Mod2.M_PI);
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

            export const M_PI = 3.14 + sin(25);

            export fn add(a, b) {
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

        // println!("{}", ir_result.ast.to_code_string());

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("Mod#M_PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod#add").is_some());

        assert!(ir_result.symbol_table.lookup("Mod2#M_PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod2#add").is_some());
    }

    #[test]
    fn test_multiple_modules_tree() {
        let main_code = "
            import Mod from \"./module.meph\";

            output a = 0;

            process {
                a = Mod.add(Mod.out, Mod.M_PI) + Mod2.add(Mod2.out, Mod2.PI);
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

            export const M_PI = 3.14 + sin(25);

            export fn add(a, b) {
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
            export const M_E = 2.71828;

            process {
                out = 42 + M_E;
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

        // println!("{}", ir_result.ast.to_code_string());

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("Mod#M_PI").is_some());
        assert!(ir_result.symbol_table.lookup("Mod#add").is_some());

        assert!(ir_result.symbol_table.lookup("Mod#Mod2#M_E").is_some());
    }

    #[test]
    fn test_multiple_modules_renaming() {
        let main_code = "
            import Mod from \"./module.meph\";
            import Mod2 from \"./module.meph\";

            param poo {
                initial: 42;
                type: C_SLIDER;
            };

            connect {
                Mod.out -> OUTPUTS;
            }

            ".to_string();

        let module_code = "
            import Lib from \"./module2.meph\";

            output out = 0;

process {
    out = Lib.M_E;
}
        ".to_string();

        let module2_code = "
            export const M_E = 2.71828;
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

        // println!("{}", ir_result.ast.to_code_string());

        ir_result.symbol_table.reset_scopes_indexes();

        assert!(ir_result.symbol_table.lookup("Mod#Lib#M_E").is_some());
    }
}
