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
                    Node::BlockNode {
                        children: _,
                        position: _,
                    }
                    |
                    Node::BufferInitializer {
                        children: _,
                        position: _,
                    }
                    |
                    Node::FunctionBody {
                        children: _,
                        position: _,
                    }
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

                    Node::ProcessNode { .. } => {
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

                    Node::ConnectNode { .. } => {
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
                                    return;
                                }

                                if context.skip_identifier_check_once {
                                    context.skip_identifier_check_once = false;
                                    return;
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
                    Node::AssignmentExpr {
                        lhs,
                        position,
                        ..
                    } => {
                        match traverse_stage {
                            ASTTraverseStage::Enter => {
                                match lhs.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        match context.symbol_table.lookup(name) {
                                            Some(symbol_info) => {
                                                if symbol_info.is_constant() {
                                                    context.errors.push(format!("Cannot assign to constant \"{}\", {:?}", name, position));
                                                }
                                            }
                                            None => {
                                                context.errors.push(format!("Cannot find name \"{}\", {:?}", name, position));
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            ASTTraverseStage::Exit => {}
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
                                    return;
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
                    _ => {}
                }
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
        _ => panic!("Expected module")
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

            bar(a, b) {
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

            baz(a, b) {
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
        assert_eq!(errors[0], "[Module \"main\"]: Function \"bar\" does not exist, Position { start: 117, end: 123, line: 8, column: 23 }");
        assert_eq!(errors[1], "[Module \"main\"]: \"foo\" is not a function, Position { start: 144, end: 150, line: 9, column: 23 }");
        assert_eq!(errors[2], "[Module \"main\"]: Function \"baz\" expects 2 arguments, but 1 were provided, Position { start: 172, end: 179, line: 11, column: 24 }");
    }

    #[test]
    fn test_undefined_fn_call_param() {
        let code = "
            let foo = 42;

            bar(a, b) {
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

        assert_eq!(errors[0], "[Module \"main\"]: Cannot find name \"c\", Position { start: 126, end: 128, line: 8, column: 27 }");
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
    #[ignore]
    fn test_connect_block() {
        let main_code = "
            import Kick from './kick.meph';

            let a = 0;

            connect {
                Kick.out -> Kick.in;
                a -> OUTPUTS[0];
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(main_code);

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

        todo!("Implement imports")
    }

    #[test]
    #[ignore]
    fn test_incorrect_connect_block() {
        let code = "
            import Kick from \"./kick.meph\";

            let a = 0;

            connect {
                b -> OUTPUTS;
                a -> c;
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
        assert_eq!(errors[0], "Cannot find name \"b\", Position { start: 108, end: 109, line: 7, column: 17 }");
        assert_eq!(errors[1], "Cannot find name \"c\", Position { start: 143, end: 144, line: 8, column: 20 }");

        todo!("Implement imports")
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
        foo() {
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

            export getSomething() {
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

        assert_eq!(errors.len(), 3);

        assert_eq!(errors[0], "[Module \"main\"]: Function \"Module.getSomethingElse\" does not exist (Cannot find name \"getSomethingElse\" in module \"./module.meph\"), Position { start: 145, end: 171, line: 6, column: 43 }");
        assert_eq!(errors[1], "[Module \"main\"]: Cannot find name \"bar\" in module \"./module.meph\", Position { start: 224, end: 235, line: 8, column: 28 }");
        assert_eq!(errors[2], "[Module \"./module.meph\"]: Cannot find name \"b\", Position { start: 164, end: 166, line: 10, column: 19 }");
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
                a -> d;
            }

            connect {
                b -> e;
            }

            connect {
                c -> f;
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
}
