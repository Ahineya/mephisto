use std::collections::HashMap;

use uuid::Uuid;

use crate::lexer::token::Position;
use crate::parser::ast::{AST, ASTTraverseStage, Node, traverse_ast, VariableSpecifier};

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolVisibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolOrigin {
    Local,
    ImportedModule { module: String },
    StandardLibrary,
}

#[derive(Debug, Clone)]
pub enum SymbolInfo {
    Variable {
        id: Uuid,
        visibility: SymbolVisibility,
        origin: SymbolOrigin,
        position: Position,
        specifier: VariableSpecifier,
        constant: bool,
    },
    Buffer {
        id: Uuid,
        visibility: SymbolVisibility,
        origin: SymbolOrigin,
        position: Position,
    },
    Parameter {
        id: Uuid,
        origin: SymbolOrigin,
        position: Position,
    },
    Function {
        id: Uuid,
        parameters: Vec<String>,
        visibility: SymbolVisibility,
        origin: SymbolOrigin,
        position: Position,
    },
    FunctionArgument {
        id: Uuid,
        origin: SymbolOrigin,
        position: Position,
    },
    ImportedModule {
        id: Uuid,
        path: String,
        position: Position,
    },
}

impl SymbolInfo {
    pub fn id(&self) -> &Uuid {
        match self {
            SymbolInfo::Variable { id, .. } => id,
            SymbolInfo::Parameter { id, .. } => id,
            SymbolInfo::Function { id, .. } => id,
            SymbolInfo::FunctionArgument { id, .. } => id,
            SymbolInfo::ImportedModule { id, .. } => id,
            SymbolInfo::Buffer { id, .. } => id,
        }
    }

    pub fn position(&mut self) -> &mut Position {
        match self {
            SymbolInfo::Variable { position, .. } => position,
            SymbolInfo::Parameter { position, .. } => position,
            SymbolInfo::Function { position, .. } => position,
            SymbolInfo::FunctionArgument { position, .. } => position,
            SymbolInfo::ImportedModule { position, .. } => position,
            SymbolInfo::Buffer { position, .. } => position,
        }
    }

    pub fn is_constant(&self) -> bool {
        match self {
            SymbolInfo::Variable { constant, .. } => *constant,
            SymbolInfo::Function { .. } => true,
            SymbolInfo::ImportedModule { .. } => true,
            SymbolInfo::Parameter { .. } => true,
            _ => false,
        }
    }

    pub fn is_private(&self) -> bool {
        match self {
            SymbolInfo::Variable { visibility, .. } => *visibility == SymbolVisibility::Private,
            SymbolInfo::Function { visibility, .. } => *visibility == SymbolVisibility::Private,
            SymbolInfo::Buffer { visibility, .. } => *visibility == SymbolVisibility::Private,
            SymbolInfo::Parameter { .. } => true,
            SymbolInfo::FunctionArgument { .. } => true,
            SymbolInfo::ImportedModule { .. } => true,
        }
    }

    pub fn is_input(&self) -> bool {
        match self {
            SymbolInfo::Variable { specifier, .. } => *specifier == VariableSpecifier::Input,
            _ => false,
        }
    }

    pub fn is_output(&self) -> bool {
        match self {
            SymbolInfo::Variable { specifier, .. } => *specifier == VariableSpecifier::Output,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    symbols: HashMap<String, SymbolInfo>,
    children: Vec<usize>,
    parent: Option<usize>,
}

impl Scope {
    pub fn symbols(&self) -> &HashMap<String, SymbolInfo> {
        &self.symbols
    }

    pub fn children(&self) -> &Vec<usize> {
        &self.children
    }

    pub fn parent(&self) -> &Option<usize> {
        &self.parent
    }
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope_index: usize,

    traversed_scopes: usize,
}

impl SymbolTable {
    pub fn current_scope_index(&self) -> usize {
        self.current_scope_index
    }

    pub fn scopes(&self) -> &Vec<Scope> {
        &self.scopes
    }

    pub fn set_current_scope_index(&mut self, index: usize) {
        self.current_scope_index = index;
    }
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

        let mut symbol_table = Self {
            scopes: vec![global_scope],
            current_scope_index: 0,
            traversed_scopes: 0,
        };

        symbol_table.define_stdlib_fn("abs", vec!["x"]);
        symbol_table.define_stdlib_fn("sqrt", vec!["x"]);
        symbol_table.define_stdlib_fn("pow", vec!["x", "y"]);
        symbol_table.define_stdlib_fn("exp", vec!["x"]);
        symbol_table.define_stdlib_fn("min", vec!["x", "y"]);
        symbol_table.define_stdlib_fn("max", vec!["x", "y"]);
        symbol_table.define_stdlib_fn("mod", vec!["x", "y"]);
        symbol_table.define_stdlib_fn("rand", vec![]);

        // Trigonometric functions
        symbol_table.define_stdlib_fn("sin", vec!["x"]);
        symbol_table.define_stdlib_fn("cos", vec!["x"]);
        symbol_table.define_stdlib_fn("tan", vec!["x"]);
        symbol_table.define_stdlib_fn("asin", vec!["x"]);
        symbol_table.define_stdlib_fn("acos", vec!["x"]);
        symbol_table.define_stdlib_fn("atan", vec!["x"]);
        symbol_table.define_stdlib_fn("atan2", vec!["x", "y"]);

        // Logarithmic functions
        symbol_table.define_stdlib_fn("log", vec!["x"]);
        symbol_table.define_stdlib_fn("log10", vec!["x"]);

        // Rounding functions
        symbol_table.define_stdlib_fn("floor", vec!["x"]);
        symbol_table.define_stdlib_fn("ceil", vec!["x"]);
        symbol_table.define_stdlib_fn("round", vec!["x"]);

        symbol_table.define_stdlib_const("PI");
        symbol_table.define_stdlib_const("E");
        symbol_table.define_stdlib_const("SR");

        symbol_table.define_stdlib_const("OUTPUTS");

        // Controls
        symbol_table.define_stdlib_const("C_TRIGGER");
        symbol_table.define_stdlib_const("C_SLIDER");

        symbol_table
    }

    pub fn get_stdlib_symbols(&self) -> Vec<(String, SymbolInfo)> {
        let mut symbols = Vec::new();

        for scope in self.scopes.iter() {
            for (name, symbol_info) in scope.symbols.iter() {
                if let SymbolInfo::Function { origin, .. } | SymbolInfo::Variable {origin, ..} = symbol_info {
                    if let SymbolOrigin::StandardLibrary = origin {
                        symbols.push((name.clone(), symbol_info.clone()));
                    }
                }
            }
        }

        symbols
    }

    fn define_stdlib_fn(&mut self, name: &str, parameters: Vec<&str>) {
        if let Ok(()) = self.insert(
            name.to_string(),
            SymbolInfo::Function {
                id: Uuid::new_v4(),
                parameters: parameters.iter().map(|s| s.to_string()).collect(),
                visibility: SymbolVisibility::Private,
                origin: SymbolOrigin::StandardLibrary,
                position: Position::new(),
            },
        ) {
            // Do nothing
        } else {
            panic!("Failed to insert \"{}\" function into symbol table", name);
        }
    }

    fn define_stdlib_const(&mut self, name: &str) {
        if let Ok(()) = self.insert(
            name.to_string(),
            SymbolInfo::Variable {
                id: Uuid::new_v4(),
                visibility: SymbolVisibility::Private,
                origin: SymbolOrigin::StandardLibrary,
                position: Position::new(),
                specifier: VariableSpecifier::Const,
                constant: true,
            },
        ) {
            // Do nothing
        } else {
            panic!("Failed to insert \"{}\" constant into symbol table", name);
        }
    }

    pub fn from_ast(ast: &mut AST) -> Result<Self, Vec<String>> {
        struct Context {
            symbol_table: SymbolTable,
            public_visibility: bool,
            errors: Vec<String>,
        }

        let mut context = Context {
            symbol_table: SymbolTable::new(),
            public_visibility: false,
            errors: Vec::new(),
        };

        traverse_ast(&mut ast.root, &mut |traverse_stage, node, context: &mut Context| {
            match node {
                |
                Node::ProcessNode {
                    children: _,
                    position: _,
                }
                |
                Node::BlockNode {
                    children: _,
                    position: _,
                } => {
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
                    position: _,
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
                    position: _,
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            if let Node::Identifier { name, position } = id.as_mut() {
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

                                let constant = match specifier {
                                    VariableSpecifier::Const => true,
                                    VariableSpecifier::Input => true,
                                    _ => false,
                                };

                                match context.symbol_table.insert(name.clone(), SymbolInfo::Variable {
                                    id: Uuid::new_v4(),
                                    visibility,
                                    constant,
                                    specifier: specifier.clone(),
                                    origin: SymbolOrigin::Local,
                                    position: position.clone(),
                                }) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        context.errors.push(err);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                Node::BufferDeclarationStmt {
                    id,
                    size: _,
                    initializer: _,
                    position: _,
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            if let Node::Identifier { name, position } = id.as_mut() {
                                let visibility = SymbolVisibility::Private;

                                let visibility = if context.public_visibility {
                                    SymbolVisibility::Public
                                } else {
                                    visibility
                                };

                                match context.symbol_table.insert(name.clone(), SymbolInfo::Buffer {
                                    id: Uuid::new_v4(),
                                    visibility,
                                    origin: SymbolOrigin::Local,
                                    position: position.clone(),
                                }) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        context.errors.push(err);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                Node::BufferInitializer {
                    ..
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            context.symbol_table.create_and_enter_scope();

                            // Add "i" as a variable to the scope
                            let visibility = SymbolVisibility::Private;
                            let origin = SymbolOrigin::Local;
                            let position = Position::new();

                            match context.symbol_table.insert("i".to_string(), SymbolInfo::Variable {
                                id: Uuid::new_v4(),
                                visibility,
                                origin,
                                position,
                                specifier: VariableSpecifier::Buffer,
                                constant: true,
                            }) {
                                Ok(_) => {}
                                Err(err) => {
                                    context.errors.push(err);
                                }
                            }
                        }
                        ASTTraverseStage::Exit => {
                            context.symbol_table.exit_scope();
                        }
                    }
                }

                Node::FunctionDeclarationStmt {
                    id,
                    params,
                    body: _,
                    position: _,
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            if let Node::Identifier { name, position } = id.as_mut() {
                                let visibility = if context.public_visibility {
                                    SymbolVisibility::Public
                                } else {
                                    SymbolVisibility::Private
                                };

                                match context.symbol_table.insert(name.clone(), SymbolInfo::Function {
                                    id: Uuid::new_v4(),
                                    parameters: params.iter().map(|param| {
                                        if let Node::FunctionParameter { id, .. } = param {
                                            if let Node::Identifier { name, .. } = id.as_ref() {
                                                name.clone()
                                            } else {
                                                panic!("[COMPILER ERROR] Expected identifier in function parameter list");
                                            }
                                        } else {
                                            panic!("[COMPILER ERROR] Expected identifier in function parameter list");
                                        }
                                    }).collect(),
                                    visibility,
                                    origin: SymbolOrigin::Local,
                                    position: position.clone(),
                                }) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        context.errors.push(err);
                                    }
                                }
                            }

                            context.symbol_table.create_and_enter_scope();

                            for param in params {
                                if let Node::FunctionParameter { id, .. } = param {
                                    if let Node::Identifier { name, position } = id.as_ref() {
                                        match context.symbol_table.insert(name.clone(), SymbolInfo::FunctionArgument {
                                            id: Uuid::new_v4(),
                                            origin: SymbolOrigin::Local,
                                            position: position.clone(),
                                        }) {
                                            Ok(_) => {}
                                            Err(err) => {
                                                context.errors.push(err);
                                            }
                                        }
                                    }
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
                    position: _
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            if let Node::Identifier { name, position } = id.as_mut() {
                                match context.symbol_table.insert(name.clone(), SymbolInfo::Parameter {
                                    id: Uuid::new_v4(),
                                    origin: SymbolOrigin::Local,
                                    position: position.clone(),
                                }) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        context.errors.push(err);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }

                Node::ImportStatement {
                    id,
                    path,
                    position: _
                } => {
                    match traverse_stage {
                        ASTTraverseStage::Enter => {
                            if let Node::Identifier { name, position } = id.as_mut() {
                                match context.symbol_table.insert(name.clone(), SymbolInfo::ImportedModule {
                                    id: Uuid::new_v4(),
                                    path: path.clone(),
                                    position: position.clone(),
                                }) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        context.errors.push(err);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            false
        }, &mut context);

        if context.errors.len() > 0 {
            return Err(context.errors);
        }

        Ok(context.symbol_table)
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
        if self.traversed_scopes >= self.scopes.len() - 1 {
            panic!("Attempted to enter a scope that doesn't exist! {}", self.traversed_scopes + 1);
        }

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

    pub fn insert(&mut self, name: String, mut info: SymbolInfo) -> Result<(), String> {
        if let Some(current_scope) = self.scopes.get_mut(self.current_scope_index) {

            // Check if the symbol already exists in the current scope
            if current_scope.symbols.contains_key(&name) {
                return Err(format!("'{}' is already declared in the current scope, {:?}", name, info.position()));
            }

            current_scope.symbols.insert(name, info);

            Ok(())
        } else {
            Err("[COMPILER ERROR]: No active scope to insert symbol".to_string())
        }
    }

    pub fn insert_into_global_scope(&mut self, name: String, mut info: SymbolInfo) -> Result<(), String> {
        if let Some(global_scope) = self.scopes.get_mut(0) {
            // Check if the symbol already exists in the global scope
            if global_scope.symbols.contains_key(&name) {
                return Err(format!("'{}' is already declared in the global scope, {:?}", name, info.position()));
            }

            global_scope.symbols.insert(name, info);

            Ok(())
        } else {
            Err("[COMPILER ERROR]: No global scope to insert symbol".to_string())
        }
    }

    pub fn rename_symbol(&mut self, symbol_id: Uuid, new_name: String) {
        let mut symbol = None;
        let mut symbol_name = None;
        let mut symbol_scope_index = None;

        for (scope_index, scope) in self.scopes.iter().enumerate() {
            for (name, info) in scope.symbols.iter() {
                if info.id() == &symbol_id {
                    symbol = Some(info.clone());
                    symbol_name = Some(name.clone());
                    symbol_scope_index = Some(scope_index);
                    break;
                }
            }
        }

        if let Some(symbol) = symbol {
            if let Some(symbol_name) = symbol_name {
                if let Some(symbol_scope_index) = symbol_scope_index {
                    self.scopes[symbol_scope_index].symbols.remove(&symbol_name);
                    self.scopes[symbol_scope_index].symbols.insert(new_name, symbol);
                }
            }
        }
    }

    pub fn move_variables_to_global_scope(&mut self, source_scope: usize) {
        let variables_to_move: Vec<String> = self.scopes[source_scope].symbols
            .iter()
            .filter_map(|(name, symbol_info)| {
                match symbol_info {
                    SymbolInfo::Variable { .. } => Some(name.clone()),
                    _ => None,
                }
            })
            .collect();

        for name in variables_to_move.iter() {
            if let Some(symbol_info) = self.scopes[source_scope].symbols.remove(name) {
                self.scopes[0].symbols.insert(name.clone(), symbol_info);
            }
        }
    }

    pub fn move_variable_to_global_scope(&mut self, name: &str, source_scope: usize) {
        if let Some(symbol_info) = self.scopes[source_scope].symbols.remove(name) {
            self.scopes[0].symbols.insert(name.to_string(), symbol_info);
        }
    }

    pub fn rename_variable(&mut self, old_name: &str, new_name: &str, scope_index: usize) {
        if let Some(symbol_info) = self.scopes[scope_index].symbols.remove(old_name) {
            self.scopes[self.current_scope_index].symbols.insert(new_name.to_string(), symbol_info);
        }
    }

    pub fn rename_global_variable(&mut self, old_name: &str, new_name: &str) {
        if let Some(symbol_info) = self.scopes[0].symbols.remove(old_name) {
            self.scopes[0].symbols.insert(new_name.to_string(), symbol_info);
        }
    }

    pub fn get_scope_symbol_names(&self, scope_index: usize) -> Vec<String> {
        self.scopes[scope_index].symbols.keys().map(|s| s.clone()).collect()
    }

    pub fn get_global_symbol_names(&self) -> Vec<String> {
        self.scopes[0].symbols.keys().map(|s| s.clone()).collect()
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

    pub fn lookup_in_scope(&mut self, name: &str, scope_index: usize) -> Option<SymbolInfo> {
        if let Some(current_scope) = self.scopes.get(scope_index) {
            if let Some(symbol) = current_scope.symbols.get(name) {
                return Some(symbol.clone());
            } else {
                // No parent scope, so we're done
                return None;
            }
        } else {
            panic!("Error: Invalid scope index {}", scope_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::lexer::token::Position;
    use crate::parser::Parser;

    use super::*;

    #[test]
    fn test_symbol_table() {

        // Symbol table creation

        let mut symbol_table = SymbolTable::new();
        symbol_table.insert(
            "foo".to_string(),
            SymbolInfo::Variable {
                id: Uuid::new_v4(),
                visibility: SymbolVisibility::Private,
                origin: SymbolOrigin::Local,
                specifier: VariableSpecifier::Let,
                position: Position::new(),
                constant: false,
            },
        ).unwrap();

        symbol_table.create_and_enter_scope();

        symbol_table.insert(
            "bar".to_string(),
            SymbolInfo::Function {
                id: Uuid::new_v4(),
                parameters: vec!["a".to_string(), "b".to_string()],
                visibility: SymbolVisibility::Private,
                origin: SymbolOrigin::Local,
                position: Position::new(),
            },
        ).unwrap();

        symbol_table.exit_scope();

        symbol_table.create_and_enter_scope();

        symbol_table.insert(
            "baz".to_string(),
            SymbolInfo::Variable {
                id: Uuid::new_v4(),
                visibility: SymbolVisibility::Private,
                origin: SymbolOrigin::Local,
                position: Position::new(),
                specifier: VariableSpecifier::Let,
                constant: false,
            },
        ).unwrap();

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
        let code = "
            let foo = 42;

            bar(function_argument_a, b) {
                return function_argument_a + b;
            }

            export let exported_variable = 42;

            process {
                let PI = 3.14;
                let result = bar(PI, 2);
                return result;
            }
        ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let mut symbol_table = SymbolTable::from_ast(&mut ast);

        assert!(symbol_table.is_ok());

        println!("{:#?}", symbol_table);

        if let Ok(symbol_table) = &mut symbol_table {
            let symbol = symbol_table.lookup("foo");
            assert!(symbol.is_some());

            let symbol = symbol_table.lookup("bar");
            assert!(symbol.is_some());

            let symbol = symbol_table.lookup("baz");
            assert!(symbol.is_none());

            // Enter the function scope
            symbol_table.enter_next_scope();

            let symbol = symbol_table.lookup("PI");
            assert!(symbol.is_some());

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

            let symbol = symbol_table.lookup("result");
            assert!(symbol.is_some());
        }
    }

    #[test]
    fn test_same_symbol() {
        let code = "
            let foo = 42;

            foo(a, b) {
                return a + b;
            }
            ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast);

        assert!(symbol_table.is_err());
    }

    #[test]
    fn test_buffer() {
        let code = "buffer b[1024];".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast);

        assert!(symbol_table.is_ok());

        let symbol_table = symbol_table.unwrap();

        println!("{:#?}", symbol_table);

        let symbol = symbol_table.lookup("b");

        assert!(symbol.is_some());
    }

    #[test]
    fn test_buffer_initializer_has_i() {
        let code = "
buffer foo[10] = |i| {
    return i * 2;
};
        ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast);

        let mut symbol_table = symbol_table.unwrap();

        symbol_table.reset_scopes_indexes();

        let symbol = symbol_table.lookup("foo");

        assert!(symbol.is_some());

        symbol_table.enter_next_scope();

        println!("{:#?}", symbol_table);

        let symbol = symbol_table.lookup("i");

        assert!(symbol.is_some());
    }

    #[test]
    fn test_constants() {
        let code = "
buffer buf[10] = |i| {
    return i * 2;
};

input inp = 42;
output out = 42;

let var = 42;
const constant = 42;

someFn() {
    return 42;
}
        ".to_string();

        let lexer = Lexer::new();
        let tokens = lexer.tokenize(code);

        let mut parser = Parser::new();
        let mut ast = parser.parse(tokens);

        let symbol_table = SymbolTable::from_ast(&mut ast);
        let mut symbol_table = symbol_table.unwrap();

        symbol_table.reset_scopes_indexes();

        let symbol = symbol_table.lookup("buf");
        assert!(symbol.is_some());
        let is_constant = symbol.unwrap().is_constant();
        assert_eq!(is_constant, false);

        let symbol = symbol_table.lookup("inp");
        assert!(symbol.is_some());
        let is_constant = symbol.unwrap().is_constant();
        assert!(is_constant);

        let symbol = symbol_table.lookup("out");
        assert!(symbol.is_some());
        let is_constant = symbol.unwrap().is_constant();
        assert_eq!(is_constant, false);

        let symbol = symbol_table.lookup("var");
        assert!(symbol.is_some());
        let is_constant = symbol.unwrap().is_constant();
        assert_eq!(is_constant, false);

        let symbol = symbol_table.lookup("constant");
        assert!(symbol.is_some());
        let is_constant = symbol.unwrap().is_constant();
        assert!(is_constant);

        symbol_table.enter_next_scope();

        let symbol = symbol_table.lookup("i");
        assert!(symbol.is_some());
        let is_constant = symbol.unwrap().is_constant();
        assert!(is_constant);

        let symbol = symbol_table.lookup("someFn");
        assert!(symbol.is_some());
        let is_constant = symbol.unwrap().is_constant();
        assert!(is_constant);
    }

    fn check_std_library_symbol(symbol_table: &SymbolTable, symbol_name: &str, params: Vec<&str>) {
        let symbol = symbol_table.lookup(symbol_name);
        assert!(symbol.is_some());
        let symbol_info = symbol.unwrap();

        if let SymbolInfo::Function {
            parameters,
            visibility,
            origin,
            ..
        } = symbol_info {
            assert_eq!(parameters.len(), params.len());

            for (i, param) in params.iter().enumerate() {
                assert_eq!(parameters[i], *param);
            }

            assert_eq!(visibility, &SymbolVisibility::Private);
            assert_eq!(origin, &SymbolOrigin::StandardLibrary);
        } else {
            panic!("Expected a function symbol");
        }
    }

    #[test]
    fn test_prepopulated_with_std_library() {
        let symbol_table = SymbolTable::new();

        check_std_library_symbol(&symbol_table, "abs", vec!["x"]);
        check_std_library_symbol(&symbol_table, "sqrt", vec!["x"]);
        check_std_library_symbol(&symbol_table, "pow", vec!["x", "y"]);
        check_std_library_symbol(&symbol_table, "exp", vec!["x"]);
        check_std_library_symbol(&symbol_table, "min", vec!["x", "y"]);
        check_std_library_symbol(&symbol_table, "max", vec!["x", "y"]);
        check_std_library_symbol(&symbol_table, "mod", vec!["x", "y"]);
        check_std_library_symbol(&symbol_table, "rand", vec![]);

        // Trigonometric functions
        check_std_library_symbol(&symbol_table, "sin", vec!["x"]);
        check_std_library_symbol(&symbol_table, "cos", vec!["x"]);
        check_std_library_symbol(&symbol_table, "tan", vec!["x"]);
        check_std_library_symbol(&symbol_table, "asin", vec!["x"]);
        check_std_library_symbol(&symbol_table, "acos", vec!["x"]);
        check_std_library_symbol(&symbol_table, "atan", vec!["x"]);
        check_std_library_symbol(&symbol_table, "atan2", vec!["x", "y"]);

        // Logarithmic functions
        check_std_library_symbol(&symbol_table, "log", vec!["x"]);
        check_std_library_symbol(&symbol_table, "log10", vec!["x"]);

        // Rounding functions
        check_std_library_symbol(&symbol_table, "floor", vec!["x"]);
        check_std_library_symbol(&symbol_table, "ceil", vec!["x"]);
        check_std_library_symbol(&symbol_table, "round", vec!["x"]);
    }
}
