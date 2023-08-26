use crate::codegen::CodeGenerator;
use crate::module_data::ModuleData;
use crate::parser::ast::{ASTTraverseStage, Node, Operator, traverse_ast, VariableSpecifier};

pub struct JSCodeGenerator;

pub struct Context {
    pub code: String,
    pub skip_identifiers: bool,
    pub skip_identifier_once: bool,

    pub errors: Vec<String>,
}

impl CodeGenerator for JSCodeGenerator {
    fn generate(&self, module: ModuleData) -> Result<String, Vec<String>> {
        let mut context = Context {
            code: String::new(),
            skip_identifiers: false,
            skip_identifier_once: false,

            errors: Vec::new(),
        };

        let mut ast = module.ast;

        traverse_ast(&mut ast.root, &mut ast_to_code, &mut context);

        Ok(context.code)
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
        Node::ProcessNode { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("{\n");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str("}\n\n");
                }
            }
        }
        Node::BlockNode { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.code.push_str("block {\n");
                }
                ASTTraverseStage::Exit => {
                    context.code.push_str("}\n\n");
                }
            }
        }
        Node::ConnectNode { .. } => {
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

                    context.code.push_str(&specifier.to_string());

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
    }

    false
}