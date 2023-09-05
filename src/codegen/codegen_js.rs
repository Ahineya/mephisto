use crate::codegen::CodeGenerator;
use crate::parser::ast::{ASTTraverseStage, Node, Operator, traverse_ast, VariableSpecifier};

use handlebars::Handlebars;
use std::collections::HashMap;
use crate::ir::IRResult;

pub struct JSCodeGenerator {
    handlebars: Handlebars<'static>,

    stdlib: HashMap<String, String>,
}

pub struct Context {
    pub code: String,

    pub code_map: HashMap<String, String>,
    pub current_block: String,

    pub parameter_declarations: Vec<String>,
    pub parameter_setters: Vec<String>,

    pub skip_identifiers: bool,
    pub skip_identifier_once: bool,

    pub errors: Vec<String>,

    pub stdlib: HashMap<String, String>,
}

impl Context {
    fn push_code(&mut self, code: &str) {
        self.code_map.get_mut(&self.current_block).unwrap().push_str(code);
    }

    fn push_implicit_connect(&mut self, code: &str) {
        self.code_map.get_mut(&"implicit_connect".to_string()).unwrap().push_str(code);
    }

    fn remove_last_char(&mut self) {
        self.code_map.get_mut(&self.current_block).unwrap().pop();
    }
    
    fn set_current_block(&mut self, block: &str) {
        self.current_block = block.to_string();
    }

    fn get_stdlib_symbol(&self, name: &str) -> String {
        // Name is guaranteed to be in the stdlib, so we can unwrap
        self.stdlib.get(name).unwrap().to_string()
    }
}

impl JSCodeGenerator {
    pub fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(handlebars::no_escape);

        handlebars
            .register_template_string("js", include_str!("templates/js-basic.hbs"))
            .unwrap();

        let mut stdlib = HashMap::new();

        stdlib.insert("abs".to_string(), "Math.abs".to_string());
        stdlib.insert("sqrt".to_string(), "Math.sqrt".to_string());
        stdlib.insert("pow".to_string(), "Math.pow".to_string());
        stdlib.insert("exp".to_string(), "Math.exp".to_string());
        stdlib.insert("min".to_string(), "Math.min".to_string());
        stdlib.insert("max".to_string(), "Math.max".to_string());
        stdlib.insert("mod".to_string(), "((a, b) => a % b)".to_string());
        stdlib.insert("rand".to_string(), "Math.random".to_string());

        // Trigonometric functions
        stdlib.insert("sin".to_string(), "Math.sin".to_string());
        stdlib.insert("cos".to_string(), "Math.cos".to_string());
        stdlib.insert("tan".to_string(), "Math.tan".to_string());
        stdlib.insert("asin".to_string(), "Math.asin".to_string());
        stdlib.insert("acos".to_string(), "Math.acos".to_string());
        stdlib.insert("atan".to_string(), "Math.atan".to_string());
        stdlib.insert("atan2".to_string(), "Math.atan2".to_string());

        // Logarithmic functions
        stdlib.insert("log".to_string(), "Math.log".to_string());
        stdlib.insert("log10".to_string(), "Math.log10".to_string());

        // Rounding functions
        stdlib.insert("floor".to_string(), "Math.floor".to_string());
        stdlib.insert("ceil".to_string(), "Math.ceil".to_string());
        stdlib.insert("round".to_string(), "Math.round".to_string());

        stdlib.insert("PI".to_string(), "Math.PI".to_string());
        stdlib.insert("E".to_string(), "Math.E".to_string());
        stdlib.insert("SR".to_string(), "sampleRate".to_string());

        // Controls
        stdlib.insert("C_TRIGGER".to_string(), "0".to_string());
        stdlib.insert("C_SLIDER".to_string(), "1".to_string());
        stdlib.insert("C_TOGGLE".to_string(), "2".to_string());

        // buffer functions

        stdlib.insert("buf_new".to_string(), "new Ringbuffer".to_string());
        stdlib.insert("buf_read".to_string(), "Rb.read".to_string());
        stdlib.insert("buf_push".to_string(), "Rb.push".to_string());
        stdlib.insert("buf_pop".to_string(), "Rb.pop".to_string());
        stdlib.insert("buf_length".to_string(), "Rb.length".to_string());
        stdlib.insert("buf_clear".to_string(), "Rb.clear".to_string());
        stdlib.insert("buf_put".to_string(), "Rb.put".to_string());
        stdlib.insert("buf_resize".to_string(), "Rb.resize".to_string());

        stdlib.insert("if".to_string(), "Std.if".to_string());
        stdlib.insert("if_else".to_string(), "Std.ifElse".to_string());

        JSCodeGenerator {
            handlebars,
            stdlib
        }
    }
}

impl CodeGenerator for JSCodeGenerator {

    fn generate(&self, ir: IRResult) -> Result<String, Vec<String>> {
        let mut context = Context {
            code: String::new(),
            code_map: HashMap::new(),
            current_block: "glob".to_string(),

            parameter_declarations: Vec::new(),
            parameter_setters: Vec::new(),

            skip_identifiers: false,
            skip_identifier_once: false,

            errors: Vec::new(),

            stdlib: self.stdlib.clone(),
        };

        context.code_map.insert("glob".to_string(), "".to_string());
        context.code_map.insert("block".to_string(), "".to_string());
        context.code_map.insert("process".to_string(), "".to_string());
        context.code_map.insert("connect".to_string(), "".to_string());
        context.code_map.insert("implicit_connect".to_string(), "".to_string());

        let mut ast = ir.ast;

        traverse_ast(&mut ast.root, &mut ast_to_code, &mut context);

        if context.errors.len() > 0 {
            return Err(context.errors);
        }

        let mut data = HashMap::new();

        let code_map = context.code_map.clone();
        let glob_code = code_map.get("glob").unwrap();
        let block_code = code_map.get("block").unwrap();
        let process_code = code_map.get("process").unwrap();
        let connect_code = code_map.get("connect").unwrap();
        let implicit_connect_code = code_map.get("implicit_connect").unwrap();

        let parameters = context.parameter_declarations.join(", ");
        let parameters = &parameters;

        let parameter_setters = context.parameter_setters.join("\n");
        let parameter_setters = &parameter_setters;

        let inputs_length = ir.input_names.len().to_string();
        let outputs_length = ir.output_names.len().to_string();

        let input_names = ir.input_names.iter().map(|name| format!("\"{}\"", name)).collect::<Vec<_>>().join(", ");
        let output_names = ir.output_names.iter().map(|name| format!("\"{}\"", name)).collect::<Vec<_>>().join(", ");

        data.insert("INPUT_NAMES", &input_names);
        data.insert("OUTPUT_NAMES", &output_names);
        data.insert("INPUTS_LENGTH", &inputs_length);
        data.insert("OUTPUTS_LENGTH", &outputs_length);
        data.insert("GLOB", &glob_code);
        data.insert("PARAMETERS", &parameters);
        data.insert("PARAMETER_SETTERS", &parameter_setters);
        data.insert("BLOCK", &block_code);
        data.insert("PROCESS", &process_code);
        data.insert("CONNECTIONS", &connect_code);
        data.insert("IMPLICIT_CONNECTIONS", &implicit_connect_code);

        let rendered = self.handlebars.render("js", &data).unwrap();

        Ok(rendered)
    }

    fn get_stdlib_symbol(&self, name: &str) -> String {
        // Name is guaranteed to be in the stdlib, so we can unwrap
        self.stdlib.get(name).unwrap().to_string()
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
                    context.set_current_block("process");
                }
                ASTTraverseStage::Exit => {
                    context.set_current_block("glob");
                }
            }
        }
        Node::BlockNode { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.set_current_block("block");
                    context.push_code("{\n");
                }
                ASTTraverseStage::Exit => {
                    context.push_code("}\n\n");
                    context.set_current_block("glob");
                }
            }
        }
        Node::ConnectNode { children, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    
                    context.set_current_block("connect");
                    
                    for child in children {
                        match child {
                            Node::ConnectStmt { lhs, rhs, .. } => {
                                // lhs is guaranteed to be Node::Identifier, so we can unwrap
                                let output_name =  match lhs.as_ref() {
                                    Node::Identifier { name, .. } => {

                                        if name.starts_with("##INPUT_") {
                                            // The string is "##INPUT_[number]". We want to extract the number
                                            let number = name.trim_start_matches("##INPUT_[").trim_end_matches("]");
                                            number.to_string()
                                        } else if name.starts_with("##OUTPUT_") {
                                            let number = name.trim_start_matches("##OUTPUT_[").trim_end_matches("]");
                                            number.to_string()
                                        } else if name.contains("#") {
                                            // replace # with __, and prepend with __
                                            let name = name.replace("#", "__");
                                            format!("##__{}", name) // Should go to implicit connections
                                        } else {
                                            format!("##{}", name.to_string()) // Should go to implicit connections
                                        }
                                    },
                                    _ => {
                                        context.errors.push("ConnectNode child not expected in the IR".to_string());
                                        return true;
                                    }
                                };

                                let input_name = match rhs.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        if name.starts_with("##INPUT_") {
                                            // The string is "##INPUT_[number]". We want to extract the number
                                            let number = name.trim_start_matches("##INPUT_[").trim_end_matches("]");
                                            number.to_string()
                                        } else if name.starts_with("##OUTPUT_") {
                                            let number = name.trim_start_matches("##OUTPUT_[").trim_end_matches("]");
                                            number.to_string()
                                        } else if name.contains("#") {
                                            // replace # with __, and prepend with __
                                            let name = name.replace("#", "__");
                                            format!("##__{}", name) // Should go to implicit connections
                                        } else {
                                            format!("##{}", name.to_string()) // Should go to implicit connections
                                        }
                                    },
                                    Node::OutputsStmt { .. } => "#OUTPUTS".to_string(),
                                    Node::OutputsNumberedStmt { value, .. } => {
                                        format!("output[{}][i]", value)
                                    }
                                    _ => {
                                        context.errors.push("ConnectNode child not expected in the IR".to_string());
                                        return true;
                                    }
                                };

                                if input_name == "#OUTPUTS" {

                                    if output_name.parse::<i32>().is_ok() {
                                        context.push_implicit_connect(&format!("leftOutput[i] = __m_outputs[{}];\n", output_name));
                                        context.push_implicit_connect(&format!("rightOutput && (rightOutput[i] = __m_outputs[{}]);\n", output_name));
                                    } else {
                                        context.push_implicit_connect(&format!("leftOutput[i] = {};\n", output_name));
                                        context.push_implicit_connect(&format!("rightOutput && (rightOutput[i] = {});\n", output_name));
                                    }


                                } else if input_name.starts_with("##") || output_name.starts_with("##") {
                                    let input_name = input_name.trim_start_matches("##");
                                    let output_name = output_name.trim_start_matches("##");

                                    // Check if input_name is a number
                                    if input_name.parse::<i32>().is_ok() {
                                        context.push_implicit_connect(&format!("__m_inputs[{}] =", input_name));
                                    } else {
                                        context.push_implicit_connect(&format!("{} =", input_name));
                                    }

                                    // Check if output_name is a number
                                    if output_name.parse::<i32>().is_ok() {
                                        context.push_implicit_connect(&format!(" __m_outputs[{}];\n", output_name));
                                    } else {
                                        context.push_implicit_connect(&format!(" {};\n", output_name));
                                    }
                                }
                                else {
                                    context.push_code(&format!("[{}, {}],\n", output_name, input_name));
                                }
                            }
                            _ => {
                                context.errors.push("ConnectNode child not expected in the IR".to_string());
                                return true;
                            }
                        };
                    }

                    return true;
                }
                ASTTraverseStage::Exit => {
                    context.push_code("\n\n");
                    context.set_current_block("glob");
                }
            }
        }
        Node::FunctionDeclarationStmt { id, params, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.skip_identifiers = true;

                    context.push_code("function ");

                    match id.as_ref() {
                        Node::Identifier { name, .. } => {
                            if name.contains("#") {
                                // replace # with __, and prepend with __
                                let name = name.replace("#", "__");
                                context.push_code(&format!("__{}", name));
                            } else {
                                context.push_code(name);
                            }
                        }
                        _ => {}
                    }

                    context.push_code("(");

                    let mut params_str = String::new();
                    for param in params {
                        match param {
                            Node::FunctionParameter { id, .. } => {
                                match id.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        if name.contains("#") {
                                            // replace # with __, and prepend with __
                                            let name = name.replace("#", "__");
                                            params_str.push_str(&format!("__{}", name));
                                        } else {
                                            params_str.push_str(&name);
                                        }

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

                    context.push_code(&params_str);
                    context.push_code(")");
                }
                ASTTraverseStage::Exit => {
                    context.push_code("\n");
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
                    context.push_code(" {\n");
                }
                ASTTraverseStage::Exit => {
                    context.push_code("}\n");
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

                    // If name starts with "##STD_", it's a stdlib function
                    if name.starts_with("##STD_") {
                        let stdlib_name = name.trim_start_matches("##STD_");
                        context.push_code(&format!("{}", context.get_stdlib_symbol(stdlib_name)));
                    } else if name.starts_with("##INPUT_") {
                        let input_name = name.trim_start_matches("##INPUT_");
                        context.push_code(&format!("__m_inputs{}", input_name))
                    } else if name.starts_with("##OUTPUT_") {
                        let output_name = name.trim_start_matches("##OUTPUT_");
                        context.push_code(&format!("__m_outputs{}", output_name))
                    } else if name.contains("#") {
                        // replace # with __, and prepend with __
                        let name = name.replace("#", "__");
                        context.push_code(&format!("__{}", name));
                    } else {
                        context.push_code(name);
                    }
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::ExpressionStmt { child, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    traverse_ast(child, &mut ast_to_code, context);
                }
                ASTTraverseStage::Exit => {

                    // If two last characters are ";\n", remove them
                    let code = context.code_map.get_mut(&context.current_block).unwrap();
                    let len = code.len();
                    if len > 2 {
                        if &code[len - 2..] == ";\n" {
                            code.pop();
                            code.pop();
                        }
                    }

                    context.push_code(";\n");
                }
            }

            return true;
        }
        Node::AssignmentExpr { lhs, rhs, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    traverse_ast(lhs, &mut ast_to_code, context);
                    context.push_code(" = ");
                    traverse_ast(rhs, &mut ast_to_code, context);
                }
                ASTTraverseStage::Exit => {
                    context.push_code(";\n");
                }
            }

            return true;
        }
        Node::ConnectStmt { lhs, rhs, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.push_code("console.trace('FIX ME, connect statement'); //");
                    traverse_ast(lhs, &mut ast_to_code, context);
                    context.push_code(" -> ");
                    traverse_ast(rhs, &mut ast_to_code, context);
                    return true;
                }
                ASTTraverseStage::Exit => {
                    context.push_code(";\n");
                }
            }
        }
        Node::ReturnStmt { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.push_code("return ");
                }
                ASTTraverseStage::Exit => {
                    context.push_code(";\n");
                }
            }
        }
        Node::VariableDeclarationStmt { id, specifier, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    match specifier {
                        VariableSpecifier::Let => {
                            context.push_code("let ");
                        }
                        VariableSpecifier::Const => {
                            context.push_code("const ");
                        }
                        VariableSpecifier::Input => {
                            context.push_code("");
                        }
                        VariableSpecifier::Output => {
                            context.push_code("");
                        }
                        VariableSpecifier::Buffer => {
                            context.push_code("let ");
                        }
                    }

                    match id.as_ref() {
                        Node::Identifier { name, .. } => {
                            if name.starts_with("##INPUT_") {
                                let input_name = name.trim_start_matches("##INPUT_");
                                context.push_code(&format!("__m_inputs{}", input_name))
                            } else if name.starts_with("##OUTPUT_") {
                                let output_name = name.trim_start_matches("##OUTPUT_");
                                context.push_code(&format!("__m_outputs{}", output_name))
                            } else if name.contains("#") {
                                // replace # with __, and prepend with __
                                let name = name.replace("#", "__");
                                context.push_code(&format!("__{}", name));
                            } else {
                                context.push_code(name);
                            }
                        }
                        _ => {}
                    }

                    context.push_code(" = ");

                    context.skip_identifier_once = true;
                }
                ASTTraverseStage::Exit => {
                    context.push_code(";\n");
                }
            }
        }
        Node::MemberExpr { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {

                    context.errors.push("MemberExpr not expected in the IR".to_string());
                    return false;

                    // traverse_ast(object, &mut ast_to_code, context);
                    // context.push_code(".");
                    // traverse_ast(property, &mut ast_to_code, context);

                    // context.skip_identifiers = true;
                }
                ASTTraverseStage::Exit => {
                    // context.skip_identifiers = false;
                }
            }
        }
        Node::ExportDeclarationStmt { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    // context.push_code("export ");
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::ParameterDeclarationStmt { id, fields, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {

                    let mut parameter_declaration = String::new();
                    let mut parameter_setter = String::new();

                    context.push_code("let ");
                    parameter_declaration.push_str("{name:");
                    parameter_setter.push_str("case ");

                    match id.as_ref() {
                        Node::Identifier { name, .. } => {

                            let name = if name.contains("#") {
                                // replace # with __, and prepend with __
                                let name = name.replace("#", "__");
                                format!("__{}", name)
                            } else {
                                name.to_string()
                            };

                            context.push_code(&name);
                            parameter_declaration.push_str(&format!("'{}'", name));
                            parameter_setter.push_str(&format!("'{}': {} = this.scheduledParameterSetters[i].value; break;", name, name));
                        }
                        _ => {}
                    }

                    context.push_code(" = ");

                    let initial_value = fields.iter().find(|field| {
                        match field {
                            Node::ParameterDeclarationField { id, .. } => {
                                match id.as_ref() {
                                    Node::Identifier { name, .. } => {
                                        name == "initial"
                                    }
                                    _ => false
                                }
                            }
                            _ => false
                        }
                    });

                    if let Some(initial_value) = initial_value {
                        match initial_value {
                            Node::ParameterDeclarationField { specifier, .. } => {
                                let specifier = match specifier.as_ref() {
                                    Node::Number { value, .. } => {
                                        value.to_string()
                                    }
                                    Node::Identifier {name, .. } => {
                                        name.to_string()
                                    }
                                    Node::UnaryExpr { op, child, .. } => {
                                        match op {
                                            Operator::Minus => {
                                                let mut code = "-".to_string();

                                                // If child is Node::Number, we can unwrap
                                                match child.as_ref() {
                                                    Node::Number { value, .. } => {
                                                        code.push_str(&value.to_string());
                                                    }
                                                    _ => {
                                                        context.errors.push("ParameterDeclarationField not expected in the IR".to_string());
                                                        return true;
                                                    }
                                                };

                                                code
                                            }
                                            _ => {
                                                context.errors.push("ParameterDeclarationField not expected in the IR".to_string());
                                                return true;
                                            }
                                        }
                                    }
                                    _ => {
                                        context.errors.push("ParameterDeclarationField not expected in the IR".to_string());
                                        return true;
                                    }
                                };

                                context.push_code(&specifier.to_string());
                            }
                            _ => {}
                        }
                    } else {
                        context.push_code("0");
                    }

                    fields.iter().for_each(|field| {
                        match field {
                            Node::ParameterDeclarationField { id, specifier, .. } => {
                                match id.as_ref() {
                                    Node::Identifier { name, .. } => {

                                        let specifier = match specifier.as_ref() {
                                            Node::Number { value, .. } => {
                                                value.to_string()
                                            }
                                            Node::Identifier {name, .. } => {
                                                if name.starts_with("##STD_") {
                                                    let stdlib_name = name.trim_start_matches("##STD_");
                                                    context.get_stdlib_symbol(stdlib_name)
                                                } else {
                                                    name.to_string()
                                                }
                                            }
                                            Node::UnaryExpr { op, child, .. } => {
                                                match op {
                                                    Operator::Minus => {
                                                        let mut code = "-".to_string();

                                                        // If child is Node::Number, we can unwrap
                                                        match child.as_ref() {
                                                            Node::Number { value, .. } => {
                                                                code.push_str(&value.to_string());
                                                            }
                                                            _ => {
                                                                context.errors.push("ParameterDeclarationField not expected in the IR".to_string());
                                                            }
                                                        };

                                                        code
                                                    }
                                                    _ => {
                                                        panic!("ParameterDeclarationField not expected in the IR")
                                                    }
                                                }
                                            }
                                            _ => {
                                                panic!("ParameterDeclarationField not expected in the IR")
                                            }

                                        };

                                        parameter_declaration.push_str(&format!(",{}:{}", name, specifier));
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    });

                    context.push_code(";\n");
                    parameter_declaration.push_str("}");

                    context.parameter_declarations.push(parameter_declaration);
                    context.parameter_setters.push(parameter_setter);
                }
                ASTTraverseStage::Exit => {

                }
            }

            return true;
        }

        Node::ParameterDeclarationField{..} => {
            match enter_exit {
                ASTTraverseStage::Enter => {}
                ASTTraverseStage::Exit => {}
            }
        }

        Node::FnCallExpr { callee, args, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    traverse_ast(callee, &mut ast_to_code, context);
                    context.push_code("(");

                    let mut len = 0;

                    for arg in args {
                        traverse_ast(arg, &mut ast_to_code, context);
                        context.push_code(", ");
                        len += 1;
                    }

                    if len > 0 {
                        context.remove_last_char();
                        context.remove_last_char();
                    }

                    context.push_code(")");
                }
                ASTTraverseStage::Exit => {}
            }

            return true;
        }
        Node::Number { value, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.push_code(&value.to_string());
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::UnaryExpr { op, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    match op {
                        Operator::Plus => {
                            context.push_code("+");
                        }
                        Operator::Minus => {
                            context.push_code("-");
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
                    context.push_code("(");
                    traverse_ast(lhs, &mut ast_to_code, context);
                    match op {
                        Operator::Plus => {
                            context.push_code(" + ");
                        }
                        Operator::Minus => {
                            context.push_code(" - ");
                        }
                        Operator::Mul => {
                            context.push_code(" * ");
                        }
                        Operator::Div => {
                            context.push_code(" / ");
                        }
                        Operator::Eq => {
                            context.push_code(" == ");
                        }
                        Operator::Gt => {
                            context.push_code(" > ");
                        }
                        Operator::Lt => {
                            context.push_code(" < ");
                        }
                        Operator::Ge => {
                            context.push_code(" >= ");
                        }
                        Operator::Le => {
                            context.push_code(" <= ");
                        }
                        Operator::Ne => {
                            context.push_code(" != ");
                        }
                    }
                    traverse_ast(rhs, &mut ast_to_code, context);

                    match op {
                        Operator::Eq | Operator::Gt | Operator::Lt | Operator::Ge | Operator::Le | Operator::Ne => {
                            context.push_code(" ? 1 : 0");
                        }
                        _ => {}
                    }

                    context.push_code(")");

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
                    context.push_code("outputs[0]");
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::OutputsNumberedStmt {value, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.push_code("outputs[");
                    context.push_code(&value.to_string());
                    context.push_code("]");
                }
                ASTTraverseStage::Exit => {}
            }
        }
        Node::BufferDeclarationStmt {id, size, initializer, .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    // context.push_code("");
                    context.push_code("let ");
                    traverse_ast(id, &mut ast_to_code, context);

                    let id = match id.as_ref() {
                        Node::Identifier { name, .. } => {
                            if name.contains("#") {
                                // replace # with __, and prepend with __
                                let name = name.replace("#", "__");
                                format!("__{}", name)
                            } else {
                                name.to_string()
                            }
                        }
                        _ => {
                            context.errors.push("BufferDeclarationStmt not expected in the IR".to_string());
                            return true;
                        }
                    };

                    // context.push_code("[");
                    // traverse_ast(size, &mut ast_to_code, context);
                    // context.push_code("] = ");
                    context.push_code(" = ");
                    context.push_code("new Ringbuffer(");
                    traverse_ast(size, &mut ast_to_code, context);
                    context.push_code(");\n");


                    // If initializer is Node::Number, we can unwrap
                    let init = match initializer.as_ref() {
                        Node::Number { value, .. } => {
                            value
                        }
                        _ => {
                            &(-1f64)
                        }
                    };

                    if *init != -1f64 {
                        // context.push_code("");
                    } else {
                        context.push_code(format!("{}.setAll((i) => {{", id).as_str());
                        traverse_ast(initializer, &mut ast_to_code, context);
                        context.push_code("\n});\n");
                    }

                }
                ASTTraverseStage::Exit => {}
            }

            return true;
        }
        Node::BufferInitializer { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.push_code("");
                }
                ASTTraverseStage::Exit => {
                    context.push_code("");
                }
            }
        }
        Node::ImportStatement { .. } => {
            match enter_exit {
                ASTTraverseStage::Enter => {
                    context.errors.push("ImportStatement not expected in the IR".to_string());
                    return false;
                }
                ASTTraverseStage::Exit => {

                }
            }
        }
    }

    false
}