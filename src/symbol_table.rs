use std::collections::HashMap;

use crate::parser::ast::{AST, ASTTraverseStage, Node, traverse_ast, VariableSpecifier};

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolVisibility {
    Public,
    Private,
}

#[derive(Debug, Clone)]
pub enum SymbolOrigin {
    Local,
    ImportedModule { module: String },
}

#[derive(Debug, Clone)]
pub enum SymbolInfo {
    Variable {
        visibility: SymbolVisibility,
        origin: SymbolOrigin,
    },
    Parameter {
        origin: SymbolOrigin,
    },
    Function {
        parameters: Vec<String>,
        visibility: SymbolVisibility,
        origin: SymbolOrigin,
    },
    FunctionArgument {
        origin: SymbolOrigin,
    },
    ImportedModule {
        module: String,
    },
}

#[derive(Debug, Clone)]
pub struct Scope {
    symbols: HashMap<String, SymbolInfo>,
    children: Vec<usize>,
    parent: Option<usize>,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope_index: usize,

    traversed_scopes: usize,
}

/*
pub enum Node {
    ProgramNode {
        children: Vec<Node>,
    },
    ProcessNode {
        children: Vec<Node>,
    },
    BlockNode {
        children: Vec<Node>,
    },
    ConnectNode {
        children: Vec<Node>,
    },
    FunctionBody {
        children: Vec<Node>,
    },
    Identifier(String),
    ExpressionStmt {
        child: Box<Node>,
    },
    AssignmentExpr {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ConnectStmt {
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    ReturnStmt {
        child: Box<Node>,
    },
    VariableDeclarationStmt {
        id: Box<Node>,
        initializer: Box<Node>,
        specifier: VariableSpecifier,
    },
    FunctionDeclarationStmt {
        id: Box<Node>,
        params: Vec<Node>,
        body: Box<Node>,
    },
    MemberExpr {
        object: Box<Node>,
        property: Box<Node>,
    },
    ExportDeclarationStmt {
        declaration: Box<Node>,
    },

    ParameterDeclarationStmt {
        id: Box<Node>,
        fields: Vec<Node>,
    },

    ParameterDeclarationField {
        id: Box<Node>,
        specifier: f64,
    },

    FnCallExpr {
        id: Box<Node>,
        args: Vec<Node>,
    },

    Number(f64),
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    OutputsStmt,
    OutputsNumberedStmt(i32),
    BufferDeclarationStmt {
        id: Box<Node>,
        size: Box<Node>,
        initializer: Box<Node>,
    },
    BufferInitializer {
        children: Vec<Node>,
    },
    ImportStatement {
        id: Box<Node>,
        path: String,
    },
 */

impl SymbolTable {
    pub fn new() -> Self {
        let global_scope = Scope {
            symbols: HashMap::new(),
            children: Vec::new(),
            parent: None,
        };
        Self {
            scopes: vec![global_scope],
            current_scope_index: 0,
            traversed_scopes: 0,
        }
    }

    pub fn from_ast(ast: &mut AST) -> Self {
        struct Context {
            symbol_table: SymbolTable,
            public_visibility: bool,
        }

        let mut context = Context {
            symbol_table: SymbolTable::new(),
            public_visibility: false,
        };

        traverse_ast(&mut ast.root, &mut |traverse_stage, node, context: &mut Context| {
            match node {
                |
                Node::ProcessNode {
                    children: _,
                }
                |
                Node::BlockNode {
                    children: _,
                }
                |
                Node::BufferInitializer {
                    children: _,
                }
                => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            context.symbol_table.create_and_enter_scope();
                        }
                        ASTTraverseStage::Exit => {
                            context.symbol_table.exit_scope();
                        }
                    }
                }

                // Export declarations are always public
                // They may contain either a function or a variable
                Node::ExportDeclarationStmt {
                    declaration: _,
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            context.public_visibility = true;
                        }
                        ASTTraverseStage::Exit => {
                            context.public_visibility = false;
                        }
                    }
                }

                Node::VariableDeclarationStmt {
                    id,
                    initializer: _,
                    specifier,
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            if let Node::Identifier(name) = id.as_mut() {
                                let visibility = match specifier {
                                    VariableSpecifier::Input => SymbolVisibility::Public,
                                    VariableSpecifier::Output => SymbolVisibility::Public,
                                    _ => SymbolVisibility::Private,
                                };

                                let visibility = if context.public_visibility {
                                    SymbolVisibility::Public
                                } else {
                                    visibility
                                };

                                context.symbol_table.insert(name.clone(), SymbolInfo::Variable {
                                    visibility,
                                    origin: SymbolOrigin::Local,
                                });
                            }
                        }
                        _ => {}
                    }
                }

                Node::FunctionDeclarationStmt {
                    id,
                    params,
                    body: _,
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            if let Node::Identifier(name) = id.as_mut() {
                                let visibility = if context.public_visibility {
                                    SymbolVisibility::Public
                                } else {
                                    SymbolVisibility::Private
                                };

                                context.symbol_table.insert(name.clone(), SymbolInfo::Function {
                                    parameters: params.iter().map(|param| {
                                        if let Node::Identifier(name) = param {
                                            name.clone()
                                        } else {
                                            panic!("Expected identifier in function parameter list");
                                        }
                                    }).collect(),
                                    visibility,
                                    origin: SymbolOrigin::Local,
                                });
                            }

                            context.symbol_table.create_and_enter_scope();

                            for param in params {
                                if let Node::Identifier(name) = param {
                                    context.symbol_table.insert(name.clone(), SymbolInfo::FunctionArgument {
                                        origin: SymbolOrigin::Local,
                                    });
                                }
                            }
                        }
                        ASTTraverseStage::Exit => {
                            context.symbol_table.exit_scope();
                        }
                    }
                }

                Node::ParameterDeclarationStmt {
                    id,
                    fields: _,
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            if let Node::Identifier(name) = id.as_mut() {
                                context.symbol_table.insert(name.clone(), SymbolInfo::Parameter {
                                    origin: SymbolOrigin::Local,
                                });
                            }
                        }
                        _ => {}
                    }
                }

                Node::ImportStatement {
                    id,
                    path,
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            if let Node::Identifier(name) = id.as_mut() {
                                context.symbol_table.insert(name.clone(), SymbolInfo::ImportedModule {
                                    module: path.clone(),
                                });
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }, &mut context);

        return context.symbol_table;
    }

    pub fn create_and_enter_scope(&mut self) {
        let new_scope = Scope {
            symbols: HashMap::new(),
            children: Vec::new(),
            parent: Some(self.current_scope_index),
        };
        self.scopes.push(new_scope);

        let new_scope_index = self.scopes.len() - 1;

        // Update the children vector of the current (parent) scope
        self.scopes[self.current_scope_index].children.push(new_scope_index);

        // Update the current scope index
        self.current_scope_index = new_scope_index;
    }

    pub fn reset_scopes_indexes(&mut self) {
        self.traversed_scopes = 0;
        self.current_scope_index = 0;
    }

    pub fn enter_next_scope(&mut self) {
        self.traversed_scopes += 1;
        let new_scope_index = self.traversed_scopes;

        self.current_scope_index = new_scope_index;
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent_index) = self.scopes[self.current_scope_index].parent {
            self.current_scope_index = parent_index;
        } else {
            panic!("Attempted to exit the global scope!");
        }
    }

    pub fn insert(&mut self, name: String, info: SymbolInfo) {
        if let Some(current_scope) = self.scopes.get_mut(self.current_scope_index) {
            current_scope.symbols.insert(name, info);
        } else {
            panic!("Error: No active scope to insert symbol");
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        // Traverse the scope tree from the current scope to the global scope
        // and return the first symbol with the given name

        let mut current_scope_index = self.current_scope_index;
        loop {
            if let Some(current_scope) = self.scopes.get(current_scope_index) {
                if let Some(symbol) = current_scope.symbols.get(name) {
                    return Some(symbol);
                } else {
                    // No symbol with the given name in the current scope
                    // Try the parent scope
                    if let Some(parent_index) = current_scope.parent {
                        current_scope_index = parent_index;
                    } else {
                        // No parent scope, so we're done
                        return None;
                    }
                }
            } else {
                panic!("Error: Invalid scope index {}", current_scope_index);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::Operator;

    use super::*;

    #[test]
    fn test_symbol_table() {

        // Symbol table creation

        let mut symbol_table = SymbolTable::new();
        symbol_table.insert(
            "foo".to_string(),
            SymbolInfo::Variable {
                visibility: SymbolVisibility::Private,
                origin: SymbolOrigin::Local,
            },
        );

        symbol_table.create_and_enter_scope();

        symbol_table.insert(
            "bar".to_string(),
            SymbolInfo::Function {
                parameters: vec!["a".to_string(), "b".to_string()],
                visibility: SymbolVisibility::Private,
                origin: SymbolOrigin::Local,
            },
        );

        symbol_table.exit_scope();

        symbol_table.create_and_enter_scope();

        symbol_table.insert(
            "baz".to_string(),
            SymbolInfo::Variable {
                visibility: SymbolVisibility::Private,
                origin: SymbolOrigin::Local,
            },
        );

        symbol_table.exit_scope();

        let symbol = symbol_table.lookup("foo");
        assert!(symbol.is_some());

        let symbol = symbol_table.lookup("bar");
        assert!(symbol.is_none());

        // Symbol table traversal

        symbol_table.reset_scopes_indexes();

        let symbol = symbol_table.lookup("foo");
        assert!(symbol.is_some());

        symbol_table.enter_next_scope();

        let symbol = symbol_table.lookup("bar");
        assert!(symbol.is_some());

        symbol_table.exit_scope();

        symbol_table.enter_next_scope();

        let symbol = symbol_table.lookup("baz");
        assert!(symbol.is_some());

        symbol_table.exit_scope();

        println!("{:#?}", symbol_table);
    }

    #[test]
    fn test_from_ast() {
        let mut symbol_table = SymbolTable::new();

        let mut ast = AST {
            root: Node::ProgramNode {
                children: vec![
                    Node::VariableDeclarationStmt {
                        id: Box::new(Node::Identifier("foo".to_string())),
                        initializer: Box::new(Node::Number(42.0)),
                        specifier: VariableSpecifier::Let,
                    },
                    Node::FunctionDeclarationStmt {
                        id: Box::new(Node::Identifier("bar".to_string())),
                        params: vec![
                            Node::Identifier("function_argument_a".to_string()),
                            Node::Identifier("b".to_string()),
                        ],
                        body: Box::new(Node::FunctionBody {
                            children: vec![
                                Node::ReturnStmt {
                                    child: Box::new(Node::BinaryExpr {
                                        op: Operator::Plus,
                                        lhs: Box::new(Node::Identifier("a".to_string())),
                                        rhs: Box::new(Node::Identifier("b".to_string())),
                                    }),
                                },
                            ],
                        }),
                    },
                    Node::ExportDeclarationStmt {
                        declaration: Box::new(
                            Node::VariableDeclarationStmt {
                                id: Box::new(Node::Identifier("exported_variable".to_string())),
                                initializer: Box::new(Node::Number(42.0)),
                                specifier: VariableSpecifier::Let,
                            }
                        )
                    },
                    Node::ProcessNode {
                        children: vec![
                            Node::VariableDeclarationStmt {
                                id: Box::new(Node::Identifier("PI".to_string())),
                                initializer: Box::new(Node::Number(3.14)),
                                specifier: VariableSpecifier::Const,
                            },
                        ],
                    },
                ],
            },
            errors: vec![],
        };

        let mut symbol_table = SymbolTable::from_ast(&mut ast);

        let symbol = symbol_table.lookup("foo");
        assert!(symbol.is_some());

        let symbol = symbol_table.lookup("bar");
        assert!(symbol.is_some());

        let symbol = symbol_table.lookup("baz");
        assert!(symbol.is_none());

        // Enter the function scope
        symbol_table.enter_next_scope();

        let symbol = symbol_table.lookup("PI");
        assert!(symbol.is_none());

        let symbol = symbol_table.lookup("function_argument_a");
        assert!(symbol.is_some());

        let symbol = symbol_table.lookup("a");
        assert!(symbol.is_none()); // Checking that the function body tries to access an undefined variable

        symbol_table.exit_scope();

        // Check that the export declaration has been processed
        let symbol = symbol_table.lookup("exported_variable");
        assert!(symbol.is_some());

        let resolved_symbol = symbol.unwrap();

        if let SymbolInfo::Variable { visibility, .. } = resolved_symbol {
            assert_eq!(*visibility, SymbolVisibility::Public);
        } else {
            panic!("Expected a variable symbol");
        }

        // Enter the process block scope
        symbol_table.enter_next_scope();
        let symbol = symbol_table.lookup("PI");
        assert!(symbol.is_some());

        println!("{:#?}", symbol_table);
    }
}
