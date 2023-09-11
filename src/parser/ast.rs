use serde::Serialize;
use serde_json;

use crate::lexer::token::Position;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AST {
    pub root: Node,
    pub errors: Vec<String>,
}

pub struct Context {
    pub code: String,
    pub skip_identifiers: bool,
    pub skip_identifier_once: bool,
}

impl AST {
    pub fn new(root: Node, errors: Vec<String>) -> AST {
        AST { root, errors }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    pub fn to_code_string(&mut self) -> String {
        let mut context = Context {
            code: String::new(),
            skip_identifiers: false,
            skip_identifier_once: false,
        };

        traverse_ast(&mut self.root, &mut ast_to_code, &mut context);

        context.code
    }


    pub fn imports(&self) -> Vec<String> {
        let mut imports = Vec::new();
        traverse_ast(&mut self.root.clone(), &mut |enter_exit, node, context: &mut Vec<String>| {
            match node {
                Node::ImportStatement { path, .. } => {
                    match enter_exit {
                        ASTTraverseStage::Enter => {
                            context.push(path.clone());
                        }
                        ASTTraverseStage::Exit => {}
                    }
                }
                _ => {}
            }

            false
        }, &mut imports);
        imports
    }
    
    pub fn inputs(&self) -> Vec<String> {
        let mut inputs = Vec::new();
        traverse_ast(&mut self.root.clone(), &mut |enter_exit, node, context: &mut Vec<String>| {
            match node {
                Node::VariableDeclarationStmt { id, specifier, .. } => {
                    match enter_exit {
                        ASTTraverseStage::Enter => {
                            match specifier {
                                VariableSpecifier::Input => {
                                    match id.as_ref() {
                                        Node::Identifier { name, .. } => {
                                            context.push(name.clone());
                                        }
                                        _ => {}
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
        }, &mut inputs);
        inputs
    }
    
    pub fn outputs(&self) -> Vec<String> {
        let mut outputs = Vec::new();
        traverse_ast(&mut self.root.clone(), &mut |enter_exit, node, context: &mut Vec<String>| {
            match node {
                Node::VariableDeclarationStmt { id, specifier, .. } => {
                    match enter_exit {
                        ASTTraverseStage::Enter => {
                            match specifier {
                                VariableSpecifier::Output => {
                                    match id.as_ref() {
                                        Node::Identifier { name, .. } => {
                                            context.push(name.clone());
                                        }
                                        _ => {}
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
        }, &mut outputs);
        outputs
    }
}

fn ast_to_code(enter_exit: ASTTraverseStage, node: &mut Node, context: &mut Context) -> bool {
    match node {
        Node::ProgramNode { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {}
                ASTTraverseStage::Exit => {}
            }
        }
        Node::ProcessSection { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("process {\n");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str("}\n\n");
                }
            }
        }
        Node::BlockSection { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("block {\n");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str("}\n\n");
                }
            }
        }
        Node::ConnectSection { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("connect {\n");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str("}\n\n");
                }
            }
        }
        Node::FunctionDeclarationStmt { id, params, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.skip_identifiers = true;

                    match id.as_ref() {
                        Node::Identifier { name, .. } => {
                            context.code.push_str(name);
                        }
                        _ => {}
                    }

                    context.code.push_str("(");

                    let mut params_str = String::new();
                    for param in params {
                        match param {
                            Node::FunctionParameter { id, .. } => {
                                match id.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        params_str.push_str(&name);
                                        params_str.push_str(", ");
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }

                    if params_str.len() > 0 {
                        // Remove last comma and space
                        params_str.pop();
                        params_str.pop();
                    }

                    context.code.push_str(&params_str);
                    context.code.push_str(")");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str("\n");
                }
            }
        }
        Node::FunctionParameter { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {}
                ASTTraverseStage::Exit => {}
            }
        }
        Node::FunctionBody { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.skip_identifiers = false;
                    context.code.push_str(" {\n");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str("}\n");
                }
            }
        }
        Node::Identifier { name, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    if context.skip_identifiers {
                        return false;
                    }

                    if context.skip_identifier_once {
                        context.skip_identifiers = false;
                        context.skip_identifier_once = false;
                        return false;
                    }

                    context.code.push_str(&name);
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::ExpressionStmt { .. } => {}
        Node::AssignmentExpr { lhs, rhs, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    traverse_ast(lhs, &mut ast_to_code, context);
                    context.code.push_str(" = ");
                    traverse_ast(rhs, &mut ast_to_code, context);
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str(";\n");
                }
            }

            return true;
        }
        Node::ConnectStmt { lhs, rhs, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    traverse_ast(lhs, &mut ast_to_code, context);
                    context.code.push_str(" -> ");
                    traverse_ast(rhs, &mut ast_to_code, context);
                    return true;
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str(";\n");
                }
            }
        }
        Node::ReturnStmt { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("return ");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str(";\n");
                }
            }
        }
        Node::VariableDeclarationStmt { id, specifier, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    match specifier {
                        VariableSpecifier::Let => {
                            context.code.push_str("let ");
                        }
                        VariableSpecifier::Const => {
                            context.code.push_str("const ");
                        }
                        VariableSpecifier::Input => {
                            context.code.push_str("input ");
                        }
                        VariableSpecifier::Output => {
                            context.code.push_str("output ");
                        }
                        VariableSpecifier::Buffer => {
                            context.code.push_str("buffer ");
                        }
                    }

                    match id.as_ref() {
                        Node::Identifier { name, .. } => {
                            context.code.push_str(name);
                        }
                        _ => {}
                    }

                    context.code.push_str(" = ");

                    context.skip_identifier_once = true;
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str(";\n");
                }
            }
        }
        Node::MemberExpr { object, property, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    traverse_ast(object, &mut ast_to_code, context);
                    context.code.push_str(".");
                    traverse_ast(property, &mut ast_to_code, context);

                    context.skip_identifiers = true;
                }
                ASTTraverseStage::Exit => {
                    context.skip_identifiers = false;
                }
            }
        }
        Node::ExportDeclarationStmt { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("export ");
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::ParameterDeclarationStmt { id, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                   context.code.push_str("param ");

                     match id.as_ref() {
                          Node::Identifier { name, .. } => {
                            context.code.push_str(name);
                          }
                          _ => {}
                     }

                    context.code.push_str(" {\n");

                    context.skip_identifier_once = true;
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str("};\n");
                }
            }
        }
        Node::ParameterDeclarationField { id, specifier, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    match id.as_ref() {
                        Node::Identifier { name, .. } => {
                            context.code.push_str(name);
                        }
                        _ => {}
                    }

                    context.code.push_str(": ");

                    let specifier = match specifier.as_ref() {
                        Node::Identifier { name, .. } => {
                            name.to_string()
                        }
                        Node::Number { value, .. } => {
                            value.to_string()
                        }
                        Node::UnaryExpr { op, child, .. } => {
                            match op {
                                Operator::Minus => {
                                    match child.as_ref() {
                                        Node::Identifier { name, .. } => {
                                            format!("-{}", name)
                                        }
                                        Node::Number { value, .. } => {
                                            format!("-{}", value.to_string())
                                        }
                                        _ => panic!("Invalid specifier")
                                    }
                                }
                                _ => panic!("Invalid specifier")
                            }
                        }
                        _ => panic!("Invalid specifier")
                    };

                    context.code.push_str(&specifier);

                    context.code.push_str(";\n");
                }
                ASTTraverseStage::Exit => {}
            }

            return true;
        }
        Node::FnCallExpr { callee, args, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    traverse_ast(callee, &mut ast_to_code, context);
                    context.code.push_str("(");

                    for arg in args {
                        traverse_ast(arg, &mut ast_to_code, context);
                        context.code.push_str(", ");
                    }

                    // Remove last comma and space
                    context.code.pop();
                    context.code.pop();

                    context.code.push_str(")");
                }
                ASTTraverseStage::Exit => {}
            }

            return true;
        }
        Node::Number { value, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str(&value.to_string());
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::UnaryExpr { op, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    match op {
                        Operator::Plus => {
                            context.code.push_str("+");
                        }
                        Operator::Minus => {
                            context.code.push_str("-");
                        }
                        _ => {}
                    }
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::BinaryExpr { op, lhs, rhs, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("(");
                    traverse_ast(lhs, &mut ast_to_code, context);
                    match op {
                        Operator::Plus => {
                            context.code.push_str(" + ");
                        }
                        Operator::Minus => {
                            context.code.push_str(" - ");
                        }
                        Operator::Mul => {
                            context.code.push_str(" * ");
                        }
                        Operator::Div => {
                            context.code.push_str(" / ");
                        }
                        Operator::Eq => {
                            context.code.push_str(" == ");
                        }
                        Operator::Gt => {
                            context.code.push_str(" > ");
                        }
                        Operator::Lt => {
                            context.code.push_str(" < ");
                        }
                        Operator::Ge => {
                            context.code.push_str(" >= ");
                        }
                        Operator::Le => {
                            context.code.push_str(" <= ");
                        }
                        Operator::Ne => {
                            context.code.push_str(" != ");
                        }
                    }
                    traverse_ast(rhs, &mut ast_to_code, context);
                    context.code.push_str(")");

                    context.skip_identifiers = true;
                }
                ASTTraverseStage::Exit => {
                    context.skip_identifiers = false;
                }
            }

            return true;
        }
        Node::OutputsStmt { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("OUTPUTS");
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::OutputsNumberedStmt {value, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("OUTPUTS[");
                    context.code.push_str(&value.to_string());
                    context.code.push_str("]");
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::BufferDeclarationStmt {id, size, initializer, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("buffer ");
                    traverse_ast(id, &mut ast_to_code, context);
                    context.code.push_str("[");
                    traverse_ast(size, &mut ast_to_code, context);
                    context.code.push_str("] = ");
                    traverse_ast(initializer, &mut ast_to_code, context);
                    context.code.push_str(";\n");
                }
                ASTTraverseStage::Exit => {}
            }

            return true;
        }
        Node::BufferInitializer { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("|i| {\n");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str("}");
                }
            }
        }
        Node::ImportStatement { path, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("import ");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str(" from ");
                    context.code.push_str(&path);
                    context.code.push_str(";\n");
                }
            }
        }
        Node::IfStmt { test, consequent, alternate, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("if (");
                    traverse_ast(test, &mut ast_to_code, context);
                    context.code.push_str(") ");
                    traverse_ast(consequent, &mut ast_to_code, context);

                    if let Some(alternate) = alternate {
                        context.code.push_str(" else ");
                        traverse_ast(alternate, &mut ast_to_code, context);
                    }
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::BlockStmt { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("{\n");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str("}\n");
                }
            }
        }
        Node::ConnectedExpr {..} => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("connected(");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str(")");
                }
            }
        }
    }

    false
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
    Ne,
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
    ProcessSection {
        children: Vec<Node>,
        position: Position,
    },
    BlockSection {
        children: Vec<Node>,
        position: Position,
    },
    ConnectSection {
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
        specifier: Box<Node>,
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
    IfStmt {
        test: Box<Node>,
        consequent: Box<Node>,
        alternate: Option<Box<Node>>,
        position: Position,
    },
    BlockStmt {
        children: Vec<Node>,
        position: Position,
    },
    ConnectedExpr {
        test: Box<Node>,
        position: Position,
    },
}

impl Node {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

// Oooook, I definitely need to rethink data structures here. It's what you get when learn the language on the go.
impl Node {
    pub fn position(&self) -> &Position {
        match self {
            Node::ProgramNode { position, .. } => position,
            Node::ProcessSection { position, .. } => position,
            Node::BlockSection { position, .. } => position,
            Node::ConnectSection { position, .. } => position,
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
            Node::IfStmt { position, .. } => position,
            Node::BlockStmt { position, .. } => position,
            Node::ConnectedExpr { position, .. } => position,
        }
    }

    pub fn set_end(&mut self, end: u32, column: u32) {
        match self {
            Node::ProgramNode { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::ProcessSection { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::BlockSection { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::ConnectSection { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::FunctionBody { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::Identifier { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::ExpressionStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::AssignmentExpr { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::ConnectStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::ReturnStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::VariableDeclarationStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::FunctionDeclarationStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::FunctionParameter { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::MemberExpr { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::ExportDeclarationStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::ParameterDeclarationStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::ParameterDeclarationField { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::FnCallExpr { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::Number { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::UnaryExpr { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::BinaryExpr { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::OutputsStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::OutputsNumberedStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::BufferDeclarationStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::BufferInitializer { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::ImportStatement { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::IfStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::BlockStmt { position, .. } => {
                position.end = end;
                position.column = column;
            }
            Node::ConnectedExpr { position, .. } => {
                position.end = end;
                position.column = column;
            }
        }
    }
}

pub enum ASTTraverseStage {
    Enter,
    Exit,
}

pub fn traverse_ast<Context>(node: &mut Node, f: &mut dyn FnMut(ASTTraverseStage, &mut Node, &mut Context) -> bool, context: &mut Context) {
    let should_skip = f(ASTTraverseStage::Enter, node, context);

    if !should_skip {
        match node {
            Node::ProgramNode { children, position: _ } => {
                for child in children {
                    traverse_ast(child, f, context);
                }
            }
            Node::ProcessSection { children, position: _ } => {
                for child in children {
                    traverse_ast(child, f, context);
                }
            }
            Node::BlockSection { children, position: _ } => {
                for child in children {
                    traverse_ast(child, f, context);
                }
            }
            Node::IfStmt { test, consequent, alternate, position: _ } => {
                traverse_ast(test, f, context);
                traverse_ast(consequent, f, context);
                if let Some(alternate) = alternate {
                    traverse_ast(alternate, f, context);
                }
            }
            Node::BlockStmt { children, position: _ } => {
                for child in children {
                    traverse_ast(child, f, context);
                }
            }
            Node::ConnectSection { children, position: _ } => {
                for child in children {
                    traverse_ast(child, f, context);
                }
            }
            Node::FunctionBody { children, position: _ } => {
                for child in children {
                    traverse_ast(child, f, context);
                }
            }
            Node::ExpressionStmt { child, position: _ } => {
                traverse_ast(child, f, context);
            }
            Node::AssignmentExpr { lhs, rhs, position: _ } => {
                traverse_ast(lhs, f, context);
                traverse_ast(rhs, f, context);
            }
            Node::ConnectStmt { lhs, rhs, position: _ } => {
                traverse_ast(lhs, f, context);
                traverse_ast(rhs, f, context);
            }
            Node::ReturnStmt { child, position: _ } => {
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
            Node::FunctionParameter { id, position: _ } => {
                traverse_ast(id, f, context);
            }
            Node::MemberExpr { object, property, position: _ } => {
                traverse_ast(object, f, context);
                traverse_ast(property, f, context);
            }
            Node::ExportDeclarationStmt { declaration, position: _ } => {
                traverse_ast(declaration, f, context);
            }
            Node::ParameterDeclarationStmt { id, fields, position: _ } => {
                traverse_ast(id, f, context);
                for field in fields {
                    traverse_ast(field, f, context);
                }
            }
            Node::ParameterDeclarationField { id, specifier, position: _ } => {
                traverse_ast(id, f, context);
                traverse_ast(specifier, f, context);
            }
            Node::FnCallExpr { callee, args, position: _ } => {
                traverse_ast(callee, f, context);
                for arg in args {
                    traverse_ast(arg, f, context);
                }
            }
            Node::Number { value: _, position: _ } => {}
            Node::UnaryExpr { op: _, child, position: _ } => {
                traverse_ast(child, f, context);
            }
            Node::BinaryExpr { op: _, lhs, rhs, position: _ } => {
                traverse_ast(lhs, f, context);
                traverse_ast(rhs, f, context);
            }
            Node::OutputsStmt { position: _ } => {}
            Node::OutputsNumberedStmt { value: _, position: _ } => {}
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
            Node::BufferInitializer { children, position: _ } => {
                for child in children {
                    traverse_ast(child, f, context);
                }
            }
            Node::ImportStatement { id, path: _, position: _ } => {
                traverse_ast(id, f, context);
            }
            Node::Identifier { name: _, position: _ } => {}
            Node::ConnectedExpr { test, position: _ } => {
                traverse_ast(test, f, context);
            }
        }
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
                Node::ProgramNode { children, .. } => {
                    match enter_exit {
                        ASTTraverseStage::Enter => {
                            children.push(Node::Identifier {
                                name: "on_enter".to_string(),
                                position: Position::new(),
                            });
                            context.some_vec.push("on_enter".to_string());
                        }
                        ASTTraverseStage::Exit => {
                            children.push(Node::Identifier {
                                name: "on_exit".to_string(),
                                position: Position::new(),
                            });
                            context.some_vec.push("on_exit".to_string());
                        }
                    }
                }
                _ => {}
            }

            false
        }, &mut context);

        assert_eq!(ast.root, Node::ProgramNode {
            children: vec![
                Node::Identifier {
                    name: "on_enter".to_string(),
                    position: Position::new(),
                },
                Node::Identifier {
                    name: "on_exit".to_string(),
                    position: Position::new(),
                },
            ],
            position: Position { line: 0, column: 0, start: 0, end: 0 },
        });

        assert_eq!(context.some_vec, vec!["on_enter".to_string(), "on_exit".to_string()]);
    }
}

