use indexmap::IndexMap;
use crate::module_data::ModuleData;
use crate::parser::ast::{ASTTraverseStage, Node, traverse_ast};
use crate::symbol_table::{SymbolInfo, SymbolTable};

pub struct SemanticAnalyzer {
    pub errors: Vec<String>,
}

pub struct ValidationResult {
    pub module_name: String,
    pub errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> SemanticAnalyzer {
        SemanticAnalyzer {
            errors: vec![],
        }
    }

    pub fn validate_semantics(&mut self, modules: &mut IndexMap<String, ModuleData>) -> Result<String, Vec<String>> {
        self.clear_errors();

        struct Context {
            symbol_table: SymbolTable,
            errors: Vec<String>,

            skip_identifier_check: bool,
            skip_identifier_check_once: bool,
            skip_module_check_once: bool,

            has_process_node: bool,
            has_connect_node: bool,
        }

        // For each module, traverse the AST and check for semantic errors
        let validation_result: Vec<ValidationResult> = modules.keys().into_iter().map(|module| -> ValidationResult {
            let module_data = modules.get(module).unwrap();

            let symbol_table = module_data.symbol_table.to_owned();
            let mut ast = module_data.ast.to_owned();

            // println!("{:#?}", ast);

            let mut context = Context {
                symbol_table, // TODO: Clone is expensive, / Lifetimes are hard, / Today I'm not ready, / To pull the right card.
                errors: Vec::new(),
                skip_identifier_check: false,
                skip_identifier_check_once: false,
                skip_module_check_once: false,

                has_process_node: false,
                has_connect_node: false,
            };

            traverse_ast(&mut ast.root, &mut |traverse_stage, node, context: &mut Context| {
                match node {
                    | Node::BlockSection { .. }
                    | Node::BufferInitializer { .. }
                    | Node::FunctionBody { .. }
                    | Node::BlockStmt { .. }
                    => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                context.symbol_table.enter_next_scope();
                            }
                            ASTTraverseStage::Exit => {
                                context.symbol_table.exit_scope();
                            }
                        }
                    }

                    Node::ProcessSection { .. } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                if context.has_process_node {
                                    context.errors.push(format!("Cannot have more than one process block, {:?}", node.position()));
                                }

                                context.has_process_node = true;
                                context.symbol_table.enter_next_scope();
                            }
                            ASTTraverseStage::Exit => {
                                context.symbol_table.exit_scope();
                            }
                        }
                    }

                    Node::ConnectSection { .. } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                if context.has_connect_node {
                                    context.errors.push(format!("Cannot have more than one connect block, {:?}", node.position()));
                                }

                                context.has_connect_node = true;
                            }
                            ASTTraverseStage::Exit => {}
                        }
                    }

                    Node::FunctionParameter {..}
                    |
                    Node::ParameterDeclarationField {..} => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                context.skip_identifier_check = true;
                            }
                            ASTTraverseStage::Exit => {
                                context.skip_identifier_check = false;
                            }
                        }
                    }

                    Node::Identifier {
                        name,
                        position,
                    } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                if context.skip_identifier_check {
                                    return false;
                                }

                                if context.skip_identifier_check_once {
                                    context.skip_identifier_check_once = false;
                                    return false;
                                }

                                let symbol = context.symbol_table.lookup(name);

                                match symbol {
                                    Some(_) => {}
                                    None => {
                                        context.errors.push(format!("Cannot find name \"{}\", {:?}", name, position));
                                    }
                                }
                            }
                            ASTTraverseStage::Exit => {}
                        }
                    }

                    Node::ConnectedExpr {
                        test,
                        position
                    } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                match test.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        match context.symbol_table.lookup(name) {
                                            Some(symbol_info) => {
                                                if !symbol_info.is_input() && !symbol_info.is_output() {
                                                    context.errors.push(format!("Cannot use {} in connected statement. Use either input or output. {:?}", name, position));
                                                }
                                            }
                                            None => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            ASTTraverseStage::Exit => {}
                        }
                    }

                    Node::AssignmentExpr {
                        lhs,
                        rhs,
                        position,
                        ..
                    } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                match lhs.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        let symbol = context.symbol_table.lookup(name);

                                        if let Some(symbol) = symbol {
                                            if symbol.is_constant() {
                                                context.errors.push(format!("Cannot assign to constant \"{}\", {:?}", name, position));
                                            }
                                        }
                                    }
                                    _ => {}
                                }

                                match rhs.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        match context.symbol_table.lookup(name) {
                                            Some(symbol_info) => {
                                                if let SymbolInfo::Function { .. } = symbol_info {
                                                    context.errors.push(format!("Cannot assign function \"{}\" to a variable, {:?}", name, position));
                                                }
                                            }
                                            None => {}
                                        }
                                    }
                                    Node::MemberExpr {
                                        object,
                                        property,
                                        ..
                                    } => {
                                        let object_name = match object.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let property_name = match property.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let result = lookup_module_symbol(&object_name, &property_name, &context.symbol_table, &modules);

                                        match result {
                                            Ok(symbol) => {
                                                let symbol = *symbol;

                                                if let SymbolInfo::Function { .. } = symbol {
                                                    context.errors.push(format!("Cannot assign function \"{}\" to a variable, {:?}", property_name, position));
                                                }
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            ASTTraverseStage::Exit => {}
                        }
                    }

                    Node::VariableDeclarationStmt {
                        initializer,
                        position,
                        ..
                    } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                match initializer.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        match context.symbol_table.lookup(name) {
                                            Some(symbol_info) => {
                                                if let SymbolInfo::Function { .. } = symbol_info {
                                                    context.errors.push(format!("Cannot assign function \"{}\" to a variable, {:?}", name, position));
                                                }
                                            }
                                            None => {}
                                        }
                                    }

                                    Node::MemberExpr {
                                        object,
                                        property,
                                        ..
                                    } => {
                                        let object_name = match object.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let property_name = match property.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let result = lookup_module_symbol(&object_name, &property_name, &context.symbol_table, &modules);

                                        match result {
                                            Ok(symbol) => {
                                                let symbol = *symbol;

                                                if let SymbolInfo::Function { .. } = symbol {
                                                    context.errors.push(format!("Cannot assign function \"{}\" to a variable, {:?}", property_name, position));
                                                }
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            ASTTraverseStage::Exit => {}
                        }
                    }

                    Node::BinaryExpr {
                        lhs,
                        rhs,
                        position,
                        ..
                    } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                match lhs.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        match context.symbol_table.lookup(name) {
                                            Some(symbol_info) => {
                                                if let SymbolInfo::Function { .. } = symbol_info {
                                                    context.errors.push(format!("Cannot use function \"{}\" as a variable, {:?}", name, position));
                                                }
                                            }
                                            None => {}
                                        }
                                    }
                                    _ => {}
                                }

                                match rhs.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        match context.symbol_table.lookup(name) {
                                            Some(symbol_info) => {
                                                if let SymbolInfo::Function { .. } = symbol_info {
                                                    context.errors.push(format!("Cannot use function \"{}\" as a variable, {:?}", name, position));
                                                }
                                            }
                                            None => {}
                                        }
                                    }
                                    Node::MemberExpr {
                                        object,
                                        property,
                                        ..
                                    } => {
                                        let object_name = match object.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let property_name = match property.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let result = lookup_module_symbol(&object_name, &property_name, &context.symbol_table, &modules);

                                        match result {
                                            Ok(symbol) => {
                                                let symbol = *symbol;

                                                if let SymbolInfo::Function { .. } = symbol {
                                                    context.errors.push(format!("Cannot assign function \"{}\" to a variable, {:?}", property_name, position));
                                                }
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            ASTTraverseStage::Exit => {}
                        }
                    }

                    Node::FnCallExpr {
                        callee, args, position, ..
                    } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                let function_name = match callee.as_ref() {
                                    Node::Identifier { name, .. } => name.to_string(),
                                    Node::MemberExpr {object, property, ..} => {
                                        let object_name = match object.as_ref() {
                                            Node::Identifier { name, .. } => name.to_string(),
                                            _ => panic!("Expected identifier")
                                        };

                                        let property_name = match property.as_ref() {
                                            Node::Identifier { name, .. } => name.to_string(),
                                            _ => panic!("Expected identifier")
                                        };

                                        format!("{}.{}", object_name, property_name)
                                    }
                                    _ => panic!("Expected function name")
                                };

                                let function_symbol = match callee.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        let symbol = context.symbol_table.lookup(name);

                                        match symbol {
                                            Some(symbol) => {
                                                Ok(symbol)
                                            }
                                            None => {
                                                Err("".to_string())
                                            }
                                        }
                                    },
                                    Node::MemberExpr { object, property,  .. } => {
                                        context.skip_module_check_once = true;

                                        let object_name = match object.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let property_name = match property.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let symbol = lookup_module_symbol(&object_name, &property_name, &context.symbol_table, &modules);

                                        match symbol {
                                            Ok(symbol) => {
                                                let symbol = *symbol;
                                                Ok(symbol)
                                            }
                                            Err(error) => {
                                                Err(error)
                                            }
                                        }
                                    }
                                    _ => panic!("Expected identifier")
                                };

                                match function_symbol {
                                    Ok(symbol) => {
                                        match symbol {
                                            SymbolInfo::Function {
                                                parameters,
                                                ..
                                            } => {
                                                if args.len() != parameters.len() {
                                                    context.errors.push(format!("Function \"{}\" expects {} arguments, but {} were provided, {:?}", function_name, parameters.len(), args.len(), position));
                                                }
                                            }
                                            _ => {
                                                context.errors.push(format!("\"{}\" is not a function, {:?}", function_name, position));
                                            }
                                        }
                                    }
                                    Err(error) => {

                                        if error.len() > 0 {
                                            context.errors.push(format!("Function \"{}\" does not exist ({}), {:?}", function_name, error, position));
                                        } else {
                                            context.errors.push(format!("Function \"{}\" does not exist, {:?}", function_name, position));
                                        }
                                    }
                                }

                                context.skip_identifier_check_once = true; // Needed to prevent the function name from being checked as an identifier, since it's already been checked here
                            }
                            ASTTraverseStage::Exit => {
                                context.skip_identifier_check_once = false;
                            }
                        }
                    }

                    Node::MemberExpr {
                        object,
                        property,
                        position,
                    } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                if context.skip_module_check_once {
                                    context.skip_identifier_check = true;
                                    context.skip_module_check_once = false;
                                    return false;
                                }

                                let object_name = match object.as_ref() {
                                    Node::Identifier { name, .. } => name,
                                    _ => panic!("Expected identifier")
                                };

                                let property_name = match property.as_ref() {
                                    Node::Identifier { name, .. } => name,
                                    _ => panic!("Expected identifier")
                                };

                                // let formatted = format!("{}.{}", object_name, property_name);

                                // Ok, here we need to lookup the symbol in the symbol table that corresponds to the module name.
                                // First we need to find the module name, which is the object_name.
                                // We want to look up module name in the current symbol table.

                                let result = lookup_module_symbol(&object_name, &property_name, &context.symbol_table, &modules);

                                match result {
                                    Ok(_) => {}
                                    Err(error) => {
                                        context.errors.push(format!("{}, {:?}", error, position));
                                    }
                                }

                                context.skip_identifier_check = true;
                            }
                            ASTTraverseStage::Exit => {
                                context.skip_identifier_check = false;
                            }
                        }
                    }

                    Node::ConnectStmt {
                        lhs,
                        rhs,
                        position
                    } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                match lhs.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        match context.symbol_table.lookup(name) {
                                            Some(symbol_info) => {
                                                if !symbol_info.is_output() {
                                                    context.errors.push(format!("Cannot connect \"{}\" to an input, declare it using \"output\" instead, {:?}", name, position));
                                                }
                                            }
                                            None => {}
                                        }
                                    }
                                    Node::MemberExpr {
                                        object,
                                        property,
                                        ..
                                    } => {
                                        let object_name = match object.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let property_name = match property.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let result = lookup_module_symbol(&object_name, &property_name, &context.symbol_table, &modules);

                                        match result {
                                            Ok(symbol) => {
                                                let symbol = *symbol;

                                                if !symbol.is_output() {
                                                    context.errors.push(format!("Cannot connect \"{}\" to an input, declare it using \"output\" instead, {:?}", property_name, position));
                                                }
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                    _ => {}
                                }

                                match rhs.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        match context.symbol_table.lookup(name) {
                                            Some(symbol_info) => {
                                                if !symbol_info.is_input() {
                                                    context.errors.push(format!("Cannot connect to \"{}\", declare it using \"input\" instead, {:?}", name, position));
                                                }
                                            }
                                            None => {}
                                        }
                                    }
                                    Node::MemberExpr {
                                        object,
                                        property,
                                        ..
                                    } => {
                                        let object_name = match object.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let property_name = match property.as_ref() {
                                            Node::Identifier { name, .. } => name,
                                            _ => panic!("Expected identifier")
                                        };

                                        let result = lookup_module_symbol(&object_name, &property_name, &context.symbol_table, &modules);

                                        match result {
                                            Ok(symbol) => {
                                                let symbol = *symbol;

                                                if !symbol.is_input() {
                                                    context.errors.push(format!("Cannot connect \"{}\" to an output, declare it using \"input\" instead, {:?}", property_name, position));
                                                }
                                            }
                                            Err(_) => {}
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            ASTTraverseStage::Exit => {}
                        }
                    }

                    _ => {}
                }

                false
            }, &mut context);

            // self.errors = context.errors;

            ValidationResult {
                module_name: module.to_owned(),
                errors: context.errors,
            }
        }).collect();

        let mut errors = vec![];

        for mut result in validation_result {
            if result.errors.len() > 0 {
                result.errors = result.errors.into_iter().map(|error| -> String {
                    format!("[Module \"{}\"]: {}", result.module_name, error)
                }).collect();

                errors.append(&mut result.errors);
            }
        }

        if errors.len() > 0 {
            return Err(errors);
        }

        Ok("".to_string())
    }

    fn clear_errors(&mut self) {
        self.errors = vec![];
    }
}

fn lookup_module_symbol<'a>(object_name: &str, property_name: &str, symbol_table: &SymbolTable, modules: &'a IndexMap<String, ModuleData>) -> Result<Box<&'a SymbolInfo>, String> {
    let module_symbol = symbol_table.lookup(object_name);

    if module_symbol.is_none() {
        return Err(format!("Cannot find module \"{}\"", object_name));
    }

    let module_symbol = module_symbol.unwrap();

    let module_path = match module_symbol {
        SymbolInfo::ImportedModule { path: module, .. } => module,
        _ => {
            return Err(format!("Cannot find module \"{}\"", object_name))
        }
    };

    let module_data = modules.get(module_path);

    if module_data.is_none() {
        return Err(format!("Cannot find module \"{}\"", module_path));
    }

    let module_data = module_data.unwrap();

    let symbol = module_data.symbol_table.lookup(property_name);

    if symbol.is_none() {
        return Err(format!("Cannot find name \"{}\" in module \"{}\"", property_name, module_path));
    }

    let symbol = symbol.unwrap();

    if symbol.is_private() {
        return Err(format!("Cannot access private symbol \"{}\" in module \"{}\"", property_name, module_path));
    }

    Ok(Box::new(symbol))
}

#[cfg(test)]
mod tests {
    use indexmap::IndexMap;
    use crate::lexer::Lexer;
    use crate::module_data::ModuleData;
    use crate::parser::Parser;
    use crate::semantic::SemanticAnalyzer;
    use crate::symbol_table::SymbolTable;

    #[test]
    fn test_semantic_analyzer_lotion() {
        let code = "
            let foo = 42;

            fn bar(a, b) {
                if (a > b) {
                    return a;
                }

                return a + b;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_ok());
    }

    #[test]
    fn test_undefined_fn_call() {
        let code = "
            let foo = 42;

            fn baz(a, b) {
                return a + b;
            }

            let a = bar();
            let b = foo();

            let c = baz(1);
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        let errors = result.unwrap_err();

        println!("{:#?}", errors);

        assert_eq!(errors.len(), 3);
        assert_eq!(errors[0], "[Module \"main\"]: Function \"bar\" does not exist, Position { start: 120, end: 126, line: 8, column: 23 }");
        assert_eq!(errors[1], "[Module \"main\"]: \"foo\" is not a function, Position { start: 147, end: 153, line: 9, column: 23 }");
        assert_eq!(errors[2], "[Module \"main\"]: Function \"baz\" expects 2 arguments, but 1 were provided, Position { start: 175, end: 182, line: 11, column: 24 }");
    }

    #[test]
    fn test_undefined_fn_call_param() {
        let code = "
            let foo = 42;

            fn bar(a, b) {
                return a + b;
            }

            let a = bar(foo, c); // c is undefined
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        let errors = result.unwrap_err();

        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0], "[Module \"main\"]: Cannot find name \"c\", Position { start: 129, end: 131, line: 8, column: 27 }");
    }

    #[test]
    fn test_undefined_variable() {
        let code = "
            let foo = a;
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], "[Module \"main\"]: Cannot find name \"a\", Position { start: 23, end: 25, line: 2, column: 21 }");
    }

    #[test]
    fn test_undefined_variable_2() {
        let code = "
            process {
                let foo = 2 + 12 + a;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        let errors = result.unwrap_err();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], "[Module \"main\"]: Cannot find name \"a\", Position { start: 58, end: 60, line: 3, column: 30 }");
    }

    #[test]
    fn test_should_ignore_param() {
        let code = "
            param moo {
                max: 12;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_ok());
    }

    #[test]
    fn test_connect_block() {
        let main_code = "
            import Module from \"./module.meph\";

            input in = 0;
            output out = 0;

            connect {
                Module.out -> in;
                out -> in;
                out -> OUTPUTS;
            }
            ".to_string();

        let module_code = "
            output out = 0;
        ".to_string();

        let lexer = Lexer::new();
        let main_tokens = lexer.tokenize(main_code);
        let module_tokens = lexer.tokenize(module_code);

        let mut parser = Parser::new();
        let mut main_ast = parser.parse(main_tokens);
        let mut module_ast = parser.parse(module_tokens);

        let main_symbol_table = SymbolTable::from_ast(&mut main_ast).unwrap();
        let module_symbol_table = SymbolTable::from_ast(&mut module_ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

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

        let result = semantic.validate_semantics(&mut modules);

        println!("{:#?}", result);

        assert!(result.is_ok());
    }

    #[test]
    fn test_incorrect_connect_block() {
        let main_code = "
            import Module from \"./module.meph\";

            input in = 0;
            output out = 0;

            let b = 12;

            connect {
                Module.out -> in1; // Incorrect input name
                in1 -> out; // Incorrect output name

                out -> OUTPUTS; // valid
                out -> OUTPUTS; // Duplicate OUTPUTS

                out -> in; // valid
                b -> in; // Cannot connect variable to output, declare it using \"output\" instead

                out -> b; // Cannot connect output to variable

                in -> out; // Cannot connect input to output, should be output -> input

                out -> Module.out; // Cannot connect to the output of a module, should be input -> output

                // Rule 1: Connection may only be made between an output and an (input or OUTPUTS)
                // Rule 2: Each input can accept only one connection
            }
            ".to_string();

        let module_code = "
            output out = 0;
        ".to_string();

        let lexer = Lexer::new();
        let main_tokens = lexer.tokenize(main_code);
        let module_tokens = lexer.tokenize(module_code);

        let mut parser = Parser::new();
        let mut main_ast = parser.parse(main_tokens);
        let mut module_ast = parser.parse(module_tokens);

        let main_symbol_table = SymbolTable::from_ast(&mut main_ast).unwrap();
        let module_symbol_table = SymbolTable::from_ast(&mut module_ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

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

        println!("{:#?}", modules);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        println!("{:#?}", errors);

        assert_eq!(errors.len(), 8);

        assert_eq!(errors[0], "[Module \"main\"]: Cannot find name \"in1\", Position { start: 182, end: 185, line: 10, column: 29 }");
        assert_eq!(errors[1], "[Module \"main\"]: Cannot connect to \"out\", declare it using \"input\" instead, Position { start: 227, end: 284, line: 11, column: 17 }");
        assert_eq!(errors[2], "[Module \"main\"]: Cannot find name \"in1\", Position { start: 227, end: 230, line: 11, column: 17 }");
        assert_eq!(errors[3], "[Module \"main\"]: Cannot connect \"b\" to an input, declare it using \"output\" instead, Position { start: 412, end: 513, line: 17, column: 17 }");
        assert_eq!(errors[4], "[Module \"main\"]: Cannot connect to \"b\", declare it using \"input\" instead, Position { start: 510, end: 576, line: 19, column: 17 }");
        assert_eq!(errors[5], "[Module \"main\"]: Cannot connect \"in\" to an input, declare it using \"output\" instead, Position { start: 574, end: 666, line: 21, column: 17 }");
        assert_eq!(errors[6], "[Module \"main\"]: Cannot connect to \"out\", declare it using \"input\" instead, Position { start: 574, end: 666, line: 21, column: 17 }");
        assert_eq!(errors[7], "[Module \"main\"]: Cannot connect \"out\" to an output, declare it using \"input\" instead, Position { start: 663, end: 935, line: 23, column: 13 }");
    }

    #[test]
    fn test_buffer_initializer() {
        let code = "
buffer foo[10] = |i| {
    return i * 2;
};
        ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_ok());
    }

    #[test]
    fn test_constant_assignment() {
        let code = "
        const foo = 10;
        foo = 12;
        ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        println!("{:#?}", ast);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());
    }

    #[test]
    fn test_function_assignment() {
        let code = "
        fn foo() {
            return 12;
        }

        foo = 12;
        ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());
    }

    #[test]
    fn test_input_assignment() {
        let code = "
        input foo = 0;

        foo = 10;
        ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());
    }

    #[test]
    #[ignore]
    fn test_no_fn_declaration_in_blocks() {
        let code = "
        process {
            foo() {
                return 12;
            }
        }

        block {
            foo() {
                return 12;
            }
        }
        ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_should_work_with_imports() {
        let main_code = "
            import Module from \"./module.meph\";

            let a = Module.out;
            let b = Module.getSomething();
            let c = Module.getSomethingElse();
            let d = Module.foo;
            let e = Module.bar;
            ".to_string();

        let module_code = "
            output out = 0;

            export fn getSomething() {
                return 42;
            }

            export const foo = 42;

            let a = b;
        ".to_string();

        let lexer = Lexer::new();
        let main_tokens = lexer.tokenize(main_code);
        let module_tokens = lexer.tokenize(module_code);

        let mut parser = Parser::new();
        let mut main_ast = parser.parse(main_tokens);
        let mut module_ast = parser.parse(module_tokens);

        let main_symbol_table = SymbolTable::from_ast(&mut main_ast).unwrap();
        let module_symbol_table = SymbolTable::from_ast(&mut module_ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

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

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        println!("{:#?}", errors);

        assert_eq!(errors.len(), 3);

        assert_eq!(errors[0], "[Module \"main\"]: Function \"Module.getSomethingElse\" does not exist (Cannot find name \"getSomethingElse\" in module \"./module.meph\"), Position { start: 145, end: 171, line: 6, column: 43 }");
        assert_eq!(errors[1], "[Module \"main\"]: Cannot find name \"bar\" in module \"./module.meph\", Position { start: 224, end: 235, line: 8, column: 28 }");
        assert_eq!(errors[2], "[Module \"./module.meph\"]: Cannot find name \"b\", Position { start: 167, end: 169, line: 10, column: 19 }");
    }

    #[test]
    fn test_multiple_process() {
        let code = "
            process {
                let a = 0;
            }

            process {
                let b = 0;
            }

            process {
                let c = 0;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        let mut modules = IndexMap::new();
        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0], "[Module \"main\"]: Cannot have more than one process block, Position { start: 77, end: 148, line: 6, column: 13 }");
        assert_eq!(errors[1], "[Module \"main\"]: Cannot have more than one process block, Position { start: 141, end: 204, line: 10, column: 13 }");
    }

    #[test]
    fn test_multiple_connect() {
        let code = "
            input a = 0;
            input b = 0;
            input c = 0;

            output d = 0;
            output e = 0;
            output f = 0;

            connect {
                d -> a;
            }

            connect {
                e -> b;
            }

            connect {
                f -> c;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        println!("{:#?}", ast);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let mut modules = IndexMap::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        println!("{:#?}", errors);

        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0], "[Module \"main\"]: Cannot have more than one connect block, Position { start: 229, end: 297, line: 14, column: 13 }");
        assert_eq!(errors[1], "[Module \"main\"]: Cannot have more than one connect block, Position { start: 290, end: 350, line: 18, column: 13 }");
    }

    #[test]
    fn test_const_assign() {
        let code = "
            const a = 1;
            a = 2;
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let mut modules = IndexMap::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        println!("{:#?}", errors);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], "[Module \"main\"]: Cannot assign to constant \"a\", Position { start: 38, end: 57, line: 3, column: 13 }");
    }

    #[test]
    fn test_fn_assign() {
        let code = "
            fn foo() {
                return 1;
            }

            let a = foo;
            a = foo;
            a = foo + 1;
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        println!("{:#?}", ast);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let mut modules = IndexMap::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        println!("{:#?}", errors);

        assert_eq!(errors.len(), 3);
        assert_eq!(errors[0], "[Module \"main\"]: Cannot assign function \"foo\" to a variable, Position { start: 77, end: 103, line: 6, column: 13 }");
        assert_eq!(errors[1], "[Module \"main\"]: Cannot assign function \"foo\" to a variable, Position { start: 102, end: 124, line: 7, column: 13 }");
        assert_eq!(errors[2], "[Module \"main\"]: Cannot use function \"foo\" as a variable, Position { start: 127, end: 135, line: 8, column: 20 }");
    }

    #[test]
    fn test_connected_expr() {
        let code = "
            input a = 0;

            if (connected(a)) {
                let b = 0;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        println!("{:#?}", ast);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let mut modules = IndexMap::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_connected_expr() {
        let code = "
            input a = 0;

            const m = 1;

            if (connected(m)) {
                let b = 0;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        println!("{:#?}", ast);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let mut modules = IndexMap::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());
    }

    #[test]
    fn test_assignment() {
        let code = "
            let a = 0;

            a = 1;
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let mut modules = IndexMap::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_ok());
    }

    #[test]
    fn test_assign_to_fn() {
        let code = "
            fn foo() {
                return 1;
            }

            foo = 1;
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();

        let mut modules = IndexMap::new();

        let module_data = ModuleData {
            ast,
            symbol_table,
            errors: vec![],
        };

        modules.insert("main".to_string(), module_data);

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        println!("{:#?}", errors);

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], "[Module \"main\"]: Cannot assign to constant \"foo\", Position { start: 77, end: 98, line: 6, column: 13 }");
    }

    #[test]
    fn test_fn_assign_members() {
        let main_code = "
            import Module from \"./module.meph\";

            let a = Module.out; // Should be ok
            let b = Module.foo; // Should fail
            let c = 1 + Module.foo; // Should fail
            ".to_string();

        let module_code = "
            output out = 0;

            export foo() {
                return 42;
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

        let mut semantic = SemanticAnalyzer::new();

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

        let result = semantic.validate_semantics(&mut modules);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        assert_eq!(errors.len(), 2);

        // assert_eq!(errors[0], "[Module \"main\"]: Function \"Module.getSomethingElse\" does not exist (Cannot find name \"getSomethingElse\" in module \"./module.meph\"), Position { start: 145, end: 171, line: 6, column: 43 }");
        // assert_eq!(errors[1], "[Module \"main\"]: Cannot find name \"bar\" in module \"./module.meph\", Position { start: 224, end: 235, line: 8, column: 28 }");
        // assert_eq!(errors[2], "[Module \"./module.meph\"]: Cannot find name \"b\", Position { start: 164, end: 166, line: 10, column: 19 }");
    }
}
