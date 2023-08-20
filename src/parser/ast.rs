use crate::lexer::token::Position;
use serde::Serialize;
use serde_json;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AST {
    pub root: Node,
    pub errors: Vec<String>,
}

impl AST {
    pub fn new(root: Node, errors: Vec<String>) -> AST {
        AST { root, errors }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn imports(&self) -> Vec<String> {
        let mut imports = Vec::new();
        traverse_ast(&mut self.root.clone(), &mut |enter_exit, node, context: &mut Vec<String>| {
            match node {
                Node::ImportStatement {path, .. } => {
                    match enter_exit {
                        ASTTraverseStage::Enter => {
                            context.push(path.clone());
                        }
                        ASTTraverseStage::Exit => {}
                    }
                }
                _ => {}
            }
        }, &mut imports);
        imports
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Operator {
    Plus,
    Minus,
    Mul,
    Div,
    Eq,
    Gt,
    Lt,
    Ge,
    Le,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum VariableSpecifier {
    Let,
    Const,
    Input,
    Output,
    Buffer,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Node {
    ProgramNode {
        children: Vec<Node>,
        position: Position,
    },
    ProcessNode {
        children: Vec<Node>,
        position: Position,
    },
    BlockNode {
        children: Vec<Node>,
        position: Position,
    },
    ConnectNode {
        children: Vec<Node>,
        position: Position,
    },
    FunctionBody {
        children: Vec<Node>,
        position: Position,
    },
    Identifier {
        name: String,
        position: Position,
    },
    ExpressionStmt {
        child: Box<Node>,
        position: Position,
    },
    AssignmentExpr {
        lhs: Box<Node>,
        rhs: Box<Node>,
        position: Position,
    },
    ConnectStmt {
        lhs: Box<Node>,
        rhs: Box<Node>,
        position: Position,
    },
    ReturnStmt {
        child: Box<Node>,
        position: Position,
    },
    VariableDeclarationStmt {
        id: Box<Node>,
        initializer: Box<Node>,
        specifier: VariableSpecifier,
        position: Position,
    },
    FunctionDeclarationStmt {
        id: Box<Node>,
        params: Vec<Node>,
        body: Box<Node>,
        position: Position,
    },
    FunctionParameter {
        id: Box<Node>,
        position: Position,
    },
    MemberExpr {
        object: Box<Node>,
        property: Box<Node>,
        position: Position,
    },
    ExportDeclarationStmt {
        declaration: Box<Node>,
        position: Position,
    },

    ParameterDeclarationStmt {
        id: Box<Node>,
        fields: Vec<Node>,
        position: Position,
    },

    ParameterDeclarationField {
        id: Box<Node>,
        specifier: f64,
        position: Position,
    },

    FnCallExpr {
        callee: Box<Node>,
        args: Vec<Node>,
        position: Position,
    },

    Number {
        value: f64,
        position: Position,
    },
    UnaryExpr {
        op: Operator,
        child: Box<Node>,
        position: Position,
    },
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
        position: Position,
    },
    OutputsStmt {
        position: Position,
    },
    OutputsNumberedStmt {
        value: i32,
        position: Position,
    },
    BufferDeclarationStmt {
        id: Box<Node>,
        size: Box<Node>,
        initializer: Box<Node>,
        position: Position,
    },
    BufferInitializer {
        children: Vec<Node>,
        position: Position,
    },
    ImportStatement {
        id: Box<Node>,
        path: String,
        position: Position,
    },
}

// Oooook, I definitely need to rethink data structures here. It's what you get when learn the language on the go.
impl Node {
    pub fn position(&self) -> &Position {
        match self {
            Node::ProgramNode { position, .. } => position,
            Node::ProcessNode { position, .. } => position,
            Node::BlockNode { position, .. } => position,
            Node::ConnectNode { position, .. } => position,
            Node::FunctionBody { position, .. } => position,
            Node::Identifier { position, .. } => position,
            Node::ExpressionStmt { position, .. } => position,
            Node::AssignmentExpr { position, .. } => position,
            Node::ConnectStmt { position, .. } => position,
            Node::ReturnStmt { position, .. } => position,
            Node::VariableDeclarationStmt { position, .. } => position,
            Node::FunctionDeclarationStmt { position, .. } => position,
            Node::FunctionParameter { position, .. } => position,
            Node::MemberExpr { position, .. } => position,
            Node::ExportDeclarationStmt { position, .. } => position,
            Node::ParameterDeclarationStmt { position, .. } => position,
            Node::ParameterDeclarationField { position, .. } => position,
            Node::FnCallExpr { position, .. } => position,
            Node::Number { position, .. } => position,
            Node::UnaryExpr { position, .. } => position,
            Node::BinaryExpr { position, .. } => position,
            Node::OutputsStmt { position, .. } => position,
            Node::OutputsNumberedStmt { position, .. } => position,
            Node::BufferDeclarationStmt { position, .. } => position,
            Node::BufferInitializer { position, .. } => position,
            Node::ImportStatement { position, .. } => position,
        }
    }
    
    pub fn set_end(&mut self, end: u32, column: u32) {
        match self {
            Node::ProgramNode { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::ProcessNode { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::BlockNode { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::ConnectNode { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::FunctionBody { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::Identifier { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::ExpressionStmt { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::AssignmentExpr { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::ConnectStmt { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::ReturnStmt { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::VariableDeclarationStmt { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::FunctionDeclarationStmt { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::FunctionParameter { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::MemberExpr { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::ExportDeclarationStmt { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::ParameterDeclarationStmt { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::ParameterDeclarationField { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::FnCallExpr { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::Number { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::UnaryExpr { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::BinaryExpr { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::OutputsStmt { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::OutputsNumberedStmt { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::BufferDeclarationStmt { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::BufferInitializer { position, .. } => {
                position.end = end;
                position.column = column;
            },
            Node::ImportStatement { position, .. } => {
                position.end = end;
                position.column = column;
            },

        }
    }
}

pub enum ASTTraverseStage {
    Enter,
    Exit,
}

pub fn traverse_ast<Context>(node: &mut Node, f: &mut dyn FnMut(ASTTraverseStage, &mut Node, &mut Context), context: &mut Context) {
    f(ASTTraverseStage::Enter, node, context);

    match node {
        Node::ProgramNode { children, position: _ } => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::ProcessNode { children, position: _ } => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::BlockNode { children, position: _ } => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::ConnectNode { children , position: _} => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::FunctionBody { children , position: _} => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::ExpressionStmt { child , position: _} => {
            traverse_ast(child, f, context);
        }
        Node::AssignmentExpr { lhs, rhs , position: _} => {
            traverse_ast(lhs, f, context);
            traverse_ast(rhs, f, context);
        }
        Node::ConnectStmt { lhs, rhs , position: _} => {
            traverse_ast(lhs, f, context);
            traverse_ast(rhs, f, context);
        }
        Node::ReturnStmt { child , position: _} => {
            traverse_ast(child, f, context);
        }
        Node::VariableDeclarationStmt {
            id,
            initializer,
            specifier: _,
            position: _
        } => {
            traverse_ast(id, f, context);
            traverse_ast(initializer, f, context);
        }
        Node::FunctionDeclarationStmt { id, params, body, position: _ } => {
            traverse_ast(id, f, context);
            for param in params {
                traverse_ast(param, f, context);
            }
            traverse_ast(body, f, context);
        }
        Node::FunctionParameter { id, position: _} => {
            traverse_ast(id, f, context);
        }
        Node::MemberExpr { object, property , position: _} => {
            traverse_ast(object, f, context);
            traverse_ast(property, f, context);
        }
        Node::ExportDeclarationStmt { declaration , position: _} => {
            traverse_ast(declaration, f, context);
        }
        Node::ParameterDeclarationStmt { id, fields , position: _} => {
            traverse_ast(id, f, context);
            for field in fields {
                traverse_ast(field, f, context);
            }
        }
        Node::ParameterDeclarationField { id, specifier: _ , position: _} => {
            traverse_ast(id, f, context);
        }
        Node::FnCallExpr { callee, args , position: _} => {
            traverse_ast(callee, f, context);
            for arg in args {
                traverse_ast(arg, f, context);
            }
        }
        Node::Number { value: _ , position: _} => {}
        Node::UnaryExpr { op: _, child , position: _} => {
            traverse_ast(child, f, context);
        }
        Node::BinaryExpr { op: _, lhs, rhs , position: _} => {
            traverse_ast(lhs, f, context);
            traverse_ast(rhs, f, context);
        }
        Node::OutputsStmt {position: _} => {}
        Node::OutputsNumberedStmt { value: _ , position: _} => {}
        Node::BufferDeclarationStmt {
            id,
            size,
            initializer,
            position: _
        } => {
            traverse_ast(id, f, context);
            traverse_ast(size, f, context);
            traverse_ast(initializer, f, context);
        }
        Node::BufferInitializer { children , position: _} => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::ImportStatement { id, path: _ , position: _} => {
            traverse_ast(id, f, context);
        }
        Node::Identifier { name: _ , position: _} => {}
    }

    f(ASTTraverseStage::Exit, node, context);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traverse() {
        let mut ast = AST {
            root: Node::ProgramNode {
                children: vec![],
                position: Position { line: 0, column: 0, start: 0, end: 0 },
            },
            errors: vec![],
        };

        struct Context {
            some_vec: Vec<String>,
        }

        let mut context = Context {
            some_vec: vec![]
        };

        traverse_ast(&mut ast.root, &mut |enter_exit, node, context: &mut Context| {
            match node {
                Node::ProgramNode { children, ..} => {
                    match enter_exit {
                        ASTTraverseStage::Enter => {
                            children.push(Node::Identifier {
                                name: "on_enter".to_string(), position: Position::new()
                            });
                            context.some_vec.push("on_enter".to_string());
                        }
                        ASTTraverseStage::Exit => {
                            children.push(Node::Identifier {
                                name: "on_exit".to_string(),
                                position: Position::new()
                            });
                            context.some_vec.push("on_exit".to_string());
                        }
                    }
                }
                _ => {}
            }
        }, &mut context);

        assert_eq!(ast.root, Node::ProgramNode {
            children: vec![
                Node::Identifier {
                    name: "on_enter".to_string(),
                    position: Position::new()
                },
                Node::Identifier {
                    name: "on_exit".to_string(),
                    position: Position::new()
                },
            ],
            position: Position { line: 0, column: 0, start: 0, end: 0 },
        });

        assert_eq!(context.some_vec, vec!["on_enter".to_string(), "on_exit".to_string()]);
    }
}

