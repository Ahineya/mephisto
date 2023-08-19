use crate::parser::ast::{AST, ASTTraverseStage, Node, traverse_ast};
use crate::symbol_table::{SymbolInfo, SymbolTable};

pub struct SemanticAnalyzer {
    pub errors: Vec<String>,
}

impl SemanticAnalyzer {
    pub fn new() -> SemanticAnalyzer {
        SemanticAnalyzer {
            errors: vec![],
        }
    }

    pub fn validate_semantics(&mut self, ast: &mut AST, symbol_table: &mut SymbolTable) -> Result<String, Vec<String>> {
        self.clear_errors();

        struct Context {
            symbol_table: SymbolTable,
            errors: Vec<String>,
            skip_identifier_check: bool,
            skip_identifier_check_once: bool,
        }

        let mut context = Context {
            symbol_table: symbol_table.clone(), // TODO: Clone is expensive, / Lifetimes are hard, / Today I'm not ready, / To pull the right card.
            errors: Vec::new(),
            skip_identifier_check: false,
            skip_identifier_check_once: false,
        };

        traverse_ast(&mut ast.root, &mut |traverse_stage, node, context: &mut Context| {
            match node {
                Node::ProcessNode {
                    children: _,
                    position: _,
                }
                |
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
                    id, args, position, ..
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            let function_name = match id.as_ref() {
                                Node::Identifier { name, .. } => name,
                                _ => panic!("Expected identifier")
                            };

                            let function_symbol = context.symbol_table.lookup(function_name);

                            match function_symbol {
                                Some(symbol) => {
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
                                None => {
                                    context.errors.push(format!("Function \"{}\" does not exist, {:?}", function_name, position));
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
                _ => {}
            }
        }, &mut context);

        self.errors = context.errors;

        if self.errors.len() > 0 {
            Err(self.errors.clone())
        } else {
            Ok("Semantics are valid".to_string())
        }
    }

    fn clear_errors(&mut self) {
        self.errors = vec![];
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

        let errors = result.unwrap_err();

        assert_eq!(errors.len(), 3);
        assert_eq!(errors[0], "Function \"bar\" does not exist, Position { start: 117, end: 123, line: 8, column: 23 }");
        assert_eq!(errors[1], "\"foo\" is not a function, Position { start: 144, end: 150, line: 9, column: 23 }");
        assert_eq!(errors[2], "Function \"baz\" expects 2 arguments, but 1 were provided, Position { start: 172, end: 179, line: 11, column: 24 }");
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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

        let errors = result.unwrap_err();

        assert_eq!(errors.len(), 1);

        assert_eq!(errors[0], "Cannot find name \"c\", Position { start: 126, end: 128, line: 8, column: 27 }");
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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

        assert!(result.is_err());

        let errors = result.unwrap_err();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], "Cannot find name \"a\", Position { start: 23, end: 25, line: 2, column: 21 }");
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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

        let errors = result.unwrap_err();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0], "Cannot find name \"a\", Position { start: 58, end: 60, line: 3, column: 30 }");
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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

        assert!(result.is_ok());
    }

    #[test]
    #[ignore]
    fn test_should_work_with_imports() {
        let code = "
            import Kick from './kick.meph';

            let a = Kick.out;
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

        assert!(result.is_ok());

        todo!("Implement imports")
    }

    #[ignore]
    #[test]
    fn test_connect_block() {
        let code = "
            import Kick from './kick.meph';

            let a = 0;

            connect {
                Kick.out -> Kick.in;
                a -> OUTPUTS[0];
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

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

        let mut symbol_table = SymbolTable::from_ast(&mut ast).unwrap();

        let mut semantic = SemanticAnalyzer::new();
        let result = semantic.validate_semantics(&mut ast, &mut symbol_table);

        assert!(result.is_err());
    }
}
