use std::collections::HashMap;

pub struct CodegenContext {
    pub code: String,

    pub code_map: HashMap<String, String>,
    pub current_block: String,

    pub parameter_declarations: Vec<String>,
    pub parameter_setters: Vec<String>,

    pub skip_identifiers: bool,
    pub skip_identifier_once: bool,

    pub is_setter: bool,

    pub errors: Vec<String>,

    pub stdlib: HashMap<String, String>,
}

impl CodegenContext {
    pub fn push_code(&mut self, code: &str) {
        self.code_map.get_mut(&self.current_block).unwrap().push_str(code);
    }

    pub fn push_implicit_connect(&mut self, code: &str) {
        self.code_map.get_mut(&CodeSection::ImplicitConnect.as_string()).unwrap().push_str(code);
    }

    pub fn remove_last_char(&mut self) {
        self.code_map.get_mut(&self.current_block).unwrap().pop();
    }

    pub fn set_current_block(&mut self, block: CodeSection) {
        self.current_block = block.as_string();
    }

    pub fn get_stdlib_symbol(&self, name: &str) -> String {
        // Name is guaranteed to be in the stdlib, so we can unwrap
        self.stdlib.get(name).unwrap().to_string()
    }
}

pub enum CodeSection {
    Glob,
    Block,
    Process,
    Connect,
    ImplicitConnect,
}

impl CodeSection {
    pub fn as_string(&self) -> String {
        match self {
            CodeSection::Glob => "glob".to_string(),
            CodeSection::Block => "block".to_string(),
            CodeSection::Process => "process".to_string(),
            CodeSection::Connect => "connect".to_string(),
            CodeSection::ImplicitConnect => "implicit_connect".to_string(),
        }
    }
}