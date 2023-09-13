pub mod codegen_js;
pub mod context;

use crate::ir::IRResult;

pub trait CodeGenerator {
    fn generate(&self, ir: IRResult) -> Result<String, Vec<String>>;
    fn get_stdlib_symbol(&self, name: &str) -> String;
}

pub struct StubCodeGenerator;

impl CodeGenerator for StubCodeGenerator {
    fn generate(&self, ir: IRResult) -> Result<String, Vec<String>> {
        Ok(format!("[STUB] IR: {:?}", ir))
    }
    
    fn get_stdlib_symbol(&self, name: &str) -> String {
        format!("[STUB] stdlib: {}", name)
    }
}