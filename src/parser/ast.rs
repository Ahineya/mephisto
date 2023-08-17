#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub root: Node,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum VariableSpecifier {
    Let,
    Const,
    Input,
    Output,
    Buffer,
}

#[derive(Debug, Clone, PartialEq)]
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
}

pub enum ASTTraverseStage {
    Enter,
    Exit,
}

pub fn traverse_ast<Context>(node: &mut Node, f: &mut dyn FnMut(ASTTraverseStage, &mut Node, &mut Context), context: &mut Context) {
    f(ASTTraverseStage::Enter, node, context);

    match node {
        Node::ProgramNode { children } => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::ProcessNode { children } => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::BlockNode { children } => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::ConnectNode { children } => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::FunctionBody { children } => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::ExpressionStmt { child } => {
            traverse_ast(child, f, context);
        }
        Node::AssignmentExpr { lhs, rhs } => {
            traverse_ast(lhs, f, context);
            traverse_ast(rhs, f, context);
        }
        Node::ConnectStmt { lhs, rhs } => {
            traverse_ast(lhs, f, context);
            traverse_ast(rhs, f, context);
        }
        Node::ReturnStmt { child } => {
            traverse_ast(child, f, context);
        }
        Node::VariableDeclarationStmt {
            id,
            initializer,
            specifier: _,
        } => {
            traverse_ast(id, f, context);
            traverse_ast(initializer, f, context);
        }
        Node::FunctionDeclarationStmt { id, params, body } => {
            traverse_ast(id, f, context);
            for param in params {
                traverse_ast(param, f, context);
            }
            traverse_ast(body, f, context);
        }
        Node::MemberExpr { object, property } => {
            traverse_ast(object, f, context);
            traverse_ast(property, f, context);
        }
        Node::ExportDeclarationStmt { declaration } => {
            traverse_ast(declaration, f, context);
        }
        Node::ParameterDeclarationStmt { id, fields } => {
            traverse_ast(id, f, context);
            for field in fields {
                traverse_ast(field, f, context);
            }
        }
        Node::ParameterDeclarationField { id, specifier: _ } => {
            traverse_ast(id, f, context);
        }
        Node::FnCallExpr { id, args } => {
            traverse_ast(id, f, context);
            for arg in args {
                traverse_ast(arg, f, context);
            }
        }
        Node::Number(_) => {}
        Node::UnaryExpr { op: _, child } => {
            traverse_ast(child, f, context);
        }
        Node::BinaryExpr { op: _, lhs, rhs } => {
            traverse_ast(lhs, f, context);
            traverse_ast(rhs, f, context);
        }
        Node::OutputsStmt => {}
        Node::OutputsNumberedStmt(_) => {}
        Node::BufferDeclarationStmt {
            id,
            size,
            initializer,
        } => {
            traverse_ast(id, f, context);
            traverse_ast(size, f, context);
            traverse_ast(initializer, f, context);
        }
        Node::BufferInitializer { children } => {
            for child in children {
                traverse_ast(child, f, context);
            }
        }
        Node::ImportStatement { id, path: _ } => {
            traverse_ast(id, f, context);
        }
        Node::Identifier(_) => {}
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
                children: vec![
                ],
            },
            errors: vec![]
        };

        struct Context {
            some_vec: Vec<String>,
        }

        let mut context = Context {
            some_vec: vec![]
        };

        traverse_ast(&mut ast.root, &mut |enter_exit, node, context: &mut Context| {
            match node {
                Node::ProgramNode { children } => {
                    match enter_exit {
                        ASTTraverseStage::Enter => {
                            children.push(Node::Identifier("on_enter".to_string()));
                            context.some_vec.push("on_enter".to_string());
                        }
                        ASTTraverseStage::Exit => {
                            children.push(Node::Identifier("on_exit".to_string()));
                            context.some_vec.push("on_exit".to_string());
                        }
                    }
                }
                _ => {}
            }
        }, &mut context);

        assert_eq!(ast.root, Node::ProgramNode {
            children: vec![
                Node::Identifier("on_enter".to_string()),
                Node::Identifier("on_exit".to_string()),
            ],
        });

        assert_eq!(context.some_vec, vec!["on_enter".to_string(), "on_exit".to_string()]);
    }
}

